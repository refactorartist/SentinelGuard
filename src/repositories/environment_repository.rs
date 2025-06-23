use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::{QueryBuilder, Row};
use uuid::Uuid;

use crate::{
    models::{
        environment::{
            Environment, EnvironmentCreatePayload, EnvironmentFilter, EnvironmentSortOrder,
            EnvironmentUpdatePayload,
        },
        pagination::Pagination,
    },
    repositories::base::Repository,
};

#[derive(Clone)]
pub struct EnvironmentRepository {
    pub pool: Arc<sqlx::postgres::PgPool>,
}

impl EnvironmentRepository {
    pub fn new(pool: Arc<sqlx::postgres::PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository<Environment> for EnvironmentRepository {
    type CreatePayload = EnvironmentCreatePayload;
    type UpdatePayload = EnvironmentUpdatePayload;
    type Filter = EnvironmentFilter;
    type Sort = EnvironmentSortOrder;

    async fn create(&self, item: Self::CreatePayload) -> Result<Environment, Error> {
        let environment = Environment {
            id: None,
            project_id: item.project_id.parse().unwrap(),
            name: item.name,
            description: item.description,
            enabled: item.enabled,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let created_environment = sqlx::query_as!(
            Environment,
            "INSERT INTO environment (project_id, name, description, enabled) VALUES ($1, $2, $3, $4) RETURNING id, project_id, name, description, enabled, created_at, updated_at",
            environment.project_id,
            environment.name,
            environment.description,
            environment.enabled,
        )
        .fetch_one(&*self.pool)
        .await;

        match created_environment {
            Ok(environment) => Ok(environment),
            Err(error) => match error {
                sqlx::Error::RowNotFound => Err(Error::msg("Environment not found")),
                sqlx::Error::Database(e) => {
                    let error_message = e.message();

                    match error_message {
                        s if s.contains("unique constraint") || s.contains("duplicate key") => {
                            if s.contains("idx_environment_project_id_name") {
                                Err(Error::msg("Project Id, name combination already exists"))
                            } else {
                                Err(Error::msg("No changes were made"))
                            }
                        }
                        s if s.contains("foreign key") && s.contains("project_id") => {
                            Err(Error::msg("Project not found"))
                        }
                        _ => Err(Error::msg("No changes were made")),
                    }
                }
                _ => Err(error.into()),
            },
        }
    }

    async fn read(&self, id: Uuid) -> Result<Option<Environment>, Error> {
        let environment = sqlx::query_as!(
            Environment,
            "SELECT id, project_id, name, description, enabled, created_at, updated_at FROM environment WHERE id = $1 LIMIT 1",
            id,
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(<sqlx::Error as Into<Error>>::into)?;

        if environment.is_none() {
            return Err(Error::msg("Environment not found"));
        }

        Ok(environment)
    }

    async fn update(&self, id: Uuid, update: Self::UpdatePayload) -> Result<Environment, Error> {
        let mut changes = Vec::new();

        if let Some(name) = update.name {
            changes.push(("name", name));
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

        let mut query = QueryBuilder::new("UPDATE environment SET ");

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
        query.push(" RETURNING id, project_id, name, description, enabled, created_at, updated_at");

        let result = query
            .build()
            .fetch_one(&*self.pool)
            .await
            .map(|row| Environment {
                id: row.get("id"),
                project_id: row.get("project_id"),
                name: row.get("name"),
                description: row.get("description"),
                enabled: row.get("enabled"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });

        match result {
            Ok(environment) => Ok(environment),
            Err(error) => match error {
                sqlx::Error::RowNotFound => Err(Error::msg("Environment not found")),
                sqlx::Error::Database(e) => {
                    let error_message = e.message();

                    match error_message {
                        s if s.contains("unique constraint") || s.contains("duplicate key") => {
                            if s.contains("idx_environment_project_id_name") {
                                Err(Error::msg("Project Id, name combination already exists"))
                            } else {
                                Err(Error::msg("No changes were made"))
                            }
                        }
                        s if s.contains("foreign key") && s.contains("project_id") => {
                            Err(Error::msg("Project not found"))
                        }
                        _ => Err(Error::msg("No changes were made")),
                    }
                }
                _ => Err(error.into()),
            },
        }
    }

    async fn delete(&self, id: Uuid) -> Result<bool, Error> {
        let deleted = sqlx::query!("DELETE FROM environment WHERE id = $1 RETURNING id", id,)
            .fetch_optional(&*self.pool)
            .await
            .map_err(<sqlx::Error as Into<Error>>::into)?;

        if deleted.is_none() {
            return Err(Error::msg("Environment not found"));
        }

        Ok(true)
    }

    async fn find(
        &self,
        filter: Self::Filter,
        sort: Option<Vec<Self::Sort>>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<Environment>, Error> {
        let mut query = QueryBuilder::new(
            "SELECT id, project_id, name, description, enabled, created_at, updated_at FROM environment ",
        );

        let mut conditions_list = Vec::new();

        if let Some(name) = filter.name {
            conditions_list.push(("name ILIKE ", format!("%{}%", name)));
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

        if !&conditions_list.is_empty() {
            query.push("WHERE ");
            let mut conditions = query.separated(" AND ");
            for (condition, value) in &conditions_list {
                if value.is_empty() {
                    conditions.push(condition);
                } else {
                    conditions.push(condition).push_bind_unseparated(value);
                }
            }
        }

        if let Some(project_id) = filter.project_id {
            if conditions_list.is_empty() {
                query.push("WHERE ");
            } else {
                query.push(" AND ");
            }
            let project_id = uuid::Uuid::parse_str(&project_id).unwrap();

            query.push(" project_id = ").push_bind(project_id);
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

        let environments = query
            .build()
            .fetch_all(&*self.pool)
            .await
            .map_err(<sqlx::Error as Into<Error>>::into)?
            .into_iter()
            .map(|row| Environment {
                id: row.get("id"),
                project_id: row.get("project_id"),
                name: row.get("name"),
                description: row.get("description"),
                enabled: row.get("enabled"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(environments)
    }
}
