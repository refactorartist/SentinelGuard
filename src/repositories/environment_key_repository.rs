use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use jsonwebtoken::Algorithm;
use sqlx::{QueryBuilder, Row};
use uuid::Uuid;

use std::str::FromStr;

use crate::{
    models::{
        environment_key::{
            EnvironmentKey, EnvironmentKeyCreatePayload, EnvironmentKeyFilter,
            EnvironmentKeySortOrder, EnvironmentKeyUpdatePayload,
        },
        pagination::Pagination,
    },
    repositories::base::Repository,
    utils::security::SecretsManager,
};

use crate::utils::tokens::key_builder::KeyBuilder;

#[derive(Clone)]
pub struct EnvironmentKeyRepository {
    pub pool: Arc<sqlx::postgres::PgPool>,
    pub secrets_manager: SecretsManager,
}

impl EnvironmentKeyRepository {
    pub fn new(pool: Arc<sqlx::postgres::PgPool>) -> Self {
        let secrets_manager = SecretsManager::new(true).unwrap();
        Self {
            pool,
            secrets_manager,
        }
    }

    pub async fn get_environment_key(
        self,
        environment_id: Uuid,
        algorithm: Algorithm,
    ) -> Result<String, Error> {
        let row = sqlx::query!(
            "SELECT id, environment_id, algorithm, key, active, created_at, updated_at FROM environment_key WHERE environment_id = $1 AND algorithm = $2 AND active = true LIMIT 1",
            environment_id,
            &format!("{:?}", algorithm),
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(|_| Error::msg("Database error"))?;

        let row = row.ok_or_else(|| Error::msg("Environment key not found"))?;
        let key = self.decrypt_key(row.key, environment_id)?;
        Ok(key)
    }

    pub fn generate_key(&self, algorithm: Algorithm) -> Result<String, Error> {
        let key = KeyBuilder::new().generate_key(algorithm)?;
        Ok(key.private_key_str)
    }

    pub fn generate_encrypted_key(
        &self,
        algorithm: Algorithm,
        environment_id: Uuid,
    ) -> Result<String, Error> {
        let key = self.generate_key(algorithm)?;
        let resource_id = environment_id;
        let key_encrypted = self.secrets_manager.encrypt(&key, &resource_id)?;
        Ok(key_encrypted)
    }

    pub fn decrypt_key(&self, key: String, environment_id: Uuid) -> Result<String, Error> {
        let key_decrypted = self.secrets_manager.decrypt(&key, &environment_id)?;
        Ok(key_decrypted)
    }
}

#[async_trait]
impl Repository<EnvironmentKey> for EnvironmentKeyRepository {
    type CreatePayload = EnvironmentKeyCreatePayload;
    type UpdatePayload = EnvironmentKeyUpdatePayload;
    type Filter = EnvironmentKeyFilter;
    type Sort = EnvironmentKeySortOrder;

    async fn create(&self, item: Self::CreatePayload) -> Result<EnvironmentKey, Error> {
        let algorithm = Algorithm::from_str(&item.algorithm).unwrap();
        let resource_id = Uuid::parse_str(&item.environment_id).unwrap();
        let key_encrypted = self.generate_encrypted_key(algorithm, resource_id)?;

        let row = sqlx::query!(
            "INSERT INTO environment_key (environment_id, algorithm, key, active) VALUES ($1, $2, $3, $4) RETURNING id, environment_id, algorithm, key, active, created_at, updated_at",
            Uuid::parse_str(&item.environment_id).unwrap(),
            &format!("{:?}", algorithm),
            key_encrypted,
            &item.active
        )
        .fetch_one(&*self.pool)
        .await;

        match row {
            Ok(row) => Ok(EnvironmentKey {
                id: Some(row.id),
                environment_id: row.environment_id,
                algorithm: Algorithm::from_str(&row.algorithm).unwrap(),
                active: row.active,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }),
            Err(error) => match error {
                sqlx::Error::RowNotFound => Err(Error::msg("Environment key not found")),
                sqlx::Error::Database(e) => {
                    let error_message = e.message();
                    dbg!(&error_message);
                    match error_message {
                        s if s.contains("unique constraint") || s.contains("duplicate key") => {
                            if s.contains("idx_environment_key_environment_id_algorithm") {
                                Err(Error::msg(
                                    "Environment Id and Algorithm combination already exists",
                                ))
                            } else {
                                dbg!(&error_message);
                                Err(Error::msg("No changes were made"))
                            }
                        }
                        s if s.contains("foreign key") => {
                            Err(Error::msg("Foreign key constraint failed"))
                        }
                        _ => Err(Error::msg("No changes were made")),
                    }
                }
                _ => Err(error.into()),
            },
        }
    }

    async fn read(&self, id: Uuid) -> Result<Option<EnvironmentKey>, Error> {
        let row = sqlx::query!(
            "SELECT id, environment_id, algorithm as algorithm, active, created_at, updated_at FROM environment_key WHERE id = $1 LIMIT 1",
            id,
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(<sqlx::Error as Into<Error>>::into)?;

        match row {
            Some(row) => Ok(Some(EnvironmentKey {
                id: Some(row.id),
                environment_id: row.environment_id,
                algorithm: Algorithm::from_str(&row.algorithm).unwrap(),
                active: row.active,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })),
            None => Err(Error::msg("Environment key not found")),
        }
    }

    async fn update(&self, id: Uuid, update: Self::UpdatePayload) -> Result<EnvironmentKey, Error> {
        let mut changes = Vec::new();

        if let Some(active) = update.active {
            match active {
                true => changes.push(("active = true", "".to_string())),
                false => changes.push(("active = false", "".to_string())),
            }
        }

        if changes.is_empty() {
            return Err(Error::msg("No changes to update"));
        }

        let mut query = QueryBuilder::new("UPDATE environment_key SET ");

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

        query.push(", updated_at = ").push_bind(chrono::Utc::now());
        query.push(" WHERE id = ").push_bind(id);
        query.push(" RETURNING id, environment_id, algorithm, active, created_at, updated_at");

        let result = query
            .build()
            .fetch_one(&*self.pool)
            .await
            .map(|row| EnvironmentKey {
                id: row.get("id"),
                environment_id: row.get("environment_id"),
                algorithm: Algorithm::from_str(&row.get::<String, _>("algorithm")).unwrap(),
                active: row.get("active"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });

        match result {
            Ok(environment_key) => Ok(environment_key),
            Err(error) => match error {
                sqlx::Error::RowNotFound => Err(Error::msg("Environment key not found")),
                sqlx::Error::Database(e) => {
                    let error_message = e.message();
                    match error_message {
                        s if s.contains("unique constraint") || s.contains("duplicate key") => {
                            if s.contains("idx_environment_key_environment_id_algorithm") {
                                Err(Error::msg(
                                    "Environment Id and Algorithm combination already exists",
                                ))
                            } else {
                                Err(Error::msg("No changes were made"))
                            }
                        }
                        s if s.contains("foreign key") => {
                            if s.contains("environment_key_environment_id_fkey") {
                                return Err(Error::msg("Environment not found"));
                            }

                            Err(Error::msg("No changes were made"))
                        }
                        _ => Err(Error::msg("No changes were made")),
                    }
                }
                _ => Err(error.into()),
            },
        }
    }

    async fn delete(&self, id: Uuid) -> Result<bool, Error> {
        let deleted = sqlx::query!("DELETE FROM environment_key WHERE id = $1 RETURNING id", id)
            .fetch_optional(&*self.pool)
            .await
            .map_err(<sqlx::Error as Into<Error>>::into)?;
        if deleted.is_none() {
            return Err(Error::msg("Environment key not found"));
        }
        Ok(true)
    }

    async fn find(
        &self,
        filter: Self::Filter,
        sort: Option<Vec<Self::Sort>>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<EnvironmentKey>, Error> {
        let mut query = QueryBuilder::new(
            "SELECT id, environment_id, algorithm, active, created_at, updated_at FROM environment_key ",
        );
        let mut conditions_list: Vec<(&str, String)> = Vec::new();
        if let Some(algorithm) = &filter.algorithm {
            conditions_list.push(("algorithm = $2", format!("{:?}", algorithm)));
        }
        if !conditions_list.is_empty() {
            query.push("WHERE ");
            let mut conditions = query.separated(" AND ");
            for (condition, value) in conditions_list.iter() {
                conditions.push(condition).push_bind(value);
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
        let keys = query
            .build()
            .fetch_all(&*self.pool)
            .await
            .map_err(<sqlx::Error as Into<Error>>::into)?
            .into_iter()
            .map(|row| EnvironmentKey {
                id: row.get("id"),
                environment_id: row.get("environment_id"),
                algorithm: Algorithm::from_str(&row.get::<String, _>("algorithm")).unwrap(),
                active: row.get("active"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();
        Ok(keys)
    }
}
