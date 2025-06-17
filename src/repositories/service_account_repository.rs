use std::sync::Arc;

use crate::models::service_account::{
    ServiceAccount, ServiceAccountCreatePayload, ServiceAccountFilter, ServiceAccountSortOrder,
    ServiceAccountUpdatePayload,
};
use crate::models::pagination::Pagination;
use crate::repositories::base::Repository;
use crate::utils::security::SecretsManager;
use anyhow::Error;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::QueryBuilder;
use sqlx::Row;
use uuid::Uuid;

#[derive(Clone)]
pub struct ServiceAccountRepository {
    pub pool: Arc<sqlx::postgres::PgPool>,
    pub secrets_manager: SecretsManager,
}

impl ServiceAccountRepository {
    pub fn new(pool: Arc<sqlx::postgres::PgPool>) -> Self {
        Self { pool, secrets_manager: SecretsManager::new(true).unwrap() }
    }
}

#[async_trait]
impl Repository<ServiceAccount> for ServiceAccountRepository {
    type CreatePayload = ServiceAccountCreatePayload;
    type UpdatePayload = ServiceAccountUpdatePayload;
    type Filter = ServiceAccountFilter;
    type Sort = ServiceAccountSortOrder;

    async fn create(&self, item: Self::CreatePayload) -> Result<ServiceAccount, Error> {
        let id = Uuid::new_v4();
        let service_account = ServiceAccount {
            id: Some(id),
            name: item.name,
            email: item.email,
            secret: self.secrets_manager.encrypt(&item.secret, &id)?,
            description: item.description,
            enabled: item.enabled,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let created_service_account = sqlx::query_as!(
            ServiceAccount,
            r#"
            INSERT INTO service_account (id, name, email, secret, description, enabled) 
            VALUES ($1, $2, $3, $4, $5, $6) 
            RETURNING id, name, email, secret, description, enabled, created_at, updated_at
            "#,
            service_account.id,
            service_account.name,
            service_account.email,
            service_account.secret,
            service_account.description,
            service_account.enabled,
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(<sqlx::Error as Into<Error>>::into);

        if created_service_account.is_err() {
            return Err(Error::msg(
                "Failed to create service account: ".to_owned() + &created_service_account.unwrap_err().to_string(),
            ));
        }

        Ok(created_service_account.unwrap())
    }

    async fn read(&self, id: Uuid) -> Result<Option<ServiceAccount>, Error> {
        let service_account = sqlx::query_as!(
            ServiceAccount,
            r#"
            SELECT id, name, email, secret, description, enabled, created_at, updated_at 
            FROM service_account 
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(<sqlx::Error as Into<Error>>::into)?;

        if service_account.is_none() {
            return Err(Error::msg("Service account not found"));
        }

        Ok(service_account)
    }


    async fn update(&self, id: Uuid, update: Self::UpdatePayload) -> Result<ServiceAccount, Error> {
        let mut changes = Vec::new();

        if let Some(name) = update.name {
            changes.push(("name", name));
        }

        if let Some(email) = update.email {
            changes.push(("email", email));
        }

        if let Some(secret) = update.secret {
            let encrypted_secret = self.secrets_manager.encrypt(&secret, &id)?;
            changes.push(("secret", encrypted_secret));
        }

        if let Some(description) = update.description {
            changes.push(("description", description));
        }

        if let Some(enabled) = update.enabled {
            match enabled {
                true => changes.push(("enabled = true", "".to_string())),
                false => changes.push(("enabled = false", "".to_string())),
            }
        }

        if changes.is_empty() {
            return Err(Error::msg("No changes to update"));
        }

        let mut query = QueryBuilder::new("UPDATE service_account SET ");

        let mut separated = query.separated(", ");
        for (field, value) in changes {
            if value.is_empty() {
                separated.push(field);
            } else {
                separated
                    .push(format!("{} = ", field))
                    .push_bind_unseparated(value);
            }
        }

        query.push(", updated_at = ").push_bind(Utc::now());
        query.push(" WHERE id = ").push_bind(id);
        query.push(" RETURNING id, name, email, secret, description, enabled, created_at, updated_at");

        let result = query
            .build()
            .fetch_one(&*self.pool)
            .await
            .map(|row| ServiceAccount {
                id: row.get("id"),
                name: row.get("name"),
                email: row.get("email"),
                secret: row.get("secret"),
                description: row.get("description"),
                enabled: row.get("enabled"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });

        match result {
            Ok(service_account) => Ok(service_account),
            Err(error) => match error {
                sqlx::Error::RowNotFound => Err(Error::msg("Service account not found")),
                sqlx::Error::Database(e) => {
                    let error_message = e.message();

                    match error_message {
                        s if s.contains("unique constraint") || s.contains("duplicate key") => {
                            if s.contains("idx_service_account_name") {
                                Err(Error::msg("Service account with this name already exists"))
                            } else if s.contains("idx_service_account_email") {
                                Err(Error::msg("Service account with this email already exists"))
                            } else {
                                Err(Error::msg("No changes were made"))
                            }
                        }
                        _ => Err(Error::msg("No changes were made")),
                    }
                }
                _ => Err(error.into()),
            },
        }
    }

    async fn delete(&self, id: Uuid) -> Result<bool, Error> {
        let result = sqlx::query!("DELETE FROM service_account WHERE id = $1", id)
            .execute(&*self.pool)
            .await;

        match result {
            Ok(result) => Ok(result.rows_affected() == 1),
            Err(error) => Err(error.into()),
        }
    }

    async fn find(
        &self,
        filter: Self::Filter,
        sort: Option<Vec<Self::Sort>>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<ServiceAccount>, Error> {
        let mut query = QueryBuilder::new(
            "SELECT id, name, email, secret, description, enabled, created_at, updated_at FROM service_account ",
        );

        let mut conditions_list = Vec::new();

        if let Some(name) = filter.name {
            conditions_list.push(("name ILIKE ", format!("%{}%", name)));
        }

        if let Some(email) = filter.email {
            conditions_list.push(("email ILIKE ", format!("%{}%", email)));
        }

        if let Some(description) = filter.description {
            conditions_list.push(("description ILIKE ", format!("%{}%", description)));
        }

        if let Some(enabled) = filter.enabled {
            match enabled {
                true => conditions_list.push(("enabled = true", "".to_string())),
                false => conditions_list.push(("enabled = false", "".to_string())),
            }
        }

        if !conditions_list.is_empty() {
            query.push("WHERE ");
            let mut conditions = query.separated(" AND ");
            for (condition, value) in conditions_list {
                if value.is_empty() {
                    conditions.push(condition);
                } else {
                    conditions.push(condition).push_bind_unseparated(value);
                }
            }
        }

        if let Some(sort) = sort {
            query.push(" ORDER BY ");
            let mut order_by = query.separated(", ");
            for sort in sort {
                let field = String::from(sort.field);
                order_by.push(format!("{} {}", field, sort.order));
            }
        }

        if let Some(pagination) = pagination {
            query
                .push(" LIMIT ")
                .push_bind(pagination.limit.unwrap_or(10));
            query
                .push(" OFFSET ")
                .push_bind(pagination.offset.unwrap_or(0));
        }

        let service_accounts: Vec<ServiceAccount> = query
            .build()
            .fetch_all(&*self.pool)
            .await
            .map_err(<sqlx::Error as Into<Error>>::into)?
            .into_iter()
            .map(|row| ServiceAccount {
                id: row.get("id"),
                name: row.get("name"),
                email: row.get("email"),
                secret: row.get("secret"),
                description: row.get("description"),
                enabled: row.get("enabled"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(service_accounts)
    }
}
