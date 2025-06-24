use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::{QueryBuilder, Row};
use uuid::Uuid;

use crate::{
    models::{
        pagination::Pagination,
        project_access::{
            ProjectAccess, ProjectAccessCreatePayload, ProjectAccessFilter, ProjectAccessSortOrder,
            ProjectAccessUpdatePayload,
        },
    },
    repositories::base::Repository,
};

#[derive(Clone)]
pub struct ProjectAccessRepository {
    pub pool: Arc<sqlx::postgres::PgPool>,
}

impl ProjectAccessRepository {
    pub fn new(pool: Arc<sqlx::postgres::PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository<ProjectAccess> for ProjectAccessRepository {
    type CreatePayload = ProjectAccessCreatePayload;
    type UpdatePayload = ProjectAccessUpdatePayload;
    type Filter = ProjectAccessFilter;
    type Sort = ProjectAccessSortOrder;

    async fn create(&self, item: Self::CreatePayload) -> Result<ProjectAccess, Error> {
        let project_access = ProjectAccess {
            id: None,
            project_id: item.project_id.parse().unwrap(),
            service_account_id: item.service_account_id.parse().unwrap(),
            environment_id: item.environment_id.parse().unwrap(),
            enabled: item.enabled,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let created_project_access = sqlx::query_as!(
            ProjectAccess,
            "INSERT INTO project_access (project_id, service_account_id, environment_id, enabled) VALUES ($1, $2, $3, $4) RETURNING id, project_id, service_account_id, environment_id, enabled, created_at, updated_at",
            project_access.project_id,
            project_access.service_account_id,
            project_access.environment_id,
            project_access.enabled,
        )
        .fetch_one(&*self.pool)
        .await;

        match created_project_access {
            Ok(project_access) => Ok(project_access),
            Err(error) => {
                match error {
                    sqlx::Error::RowNotFound => Err(Error::msg("Project access not found")),
                    sqlx::Error::Database(e) => {
                        let error_message = e.message();

                        match error_message {
                        s if s.contains("unique constraint") || s.contains("duplicate key") => {
                            if s.contains("idx_project_access_project_id_service_account_id_environment_id") {
                                Err(Error::msg("Project Id, Service Account Id and Environment Id combination already exists"))
                            } else {
                                Err(Error::msg("No changes were made"))
                            }
                        }
                        s if s.contains("foreign key") =>
                        {
                            if s.contains("project_access_project_id_fkey") {
                                return Err(Error::msg("Project not found"))
                            }

                            if s.contains("project_access_service_account_id_fkey") {
                                return Err(Error::msg("Service Account not found"))
                            }

                            if s.contains("project_access_environment_id_fkey") {
                                return Err(Error::msg("Environment not found"))
                            }
                            Err(Error::msg("No changes were made"))
                        }
                        _ => Err(Error::msg("No changes were made")),
                    }
                    }
                    _ => Err(error.into()),
                }
            }
        }
    }

    async fn read(&self, id: Uuid) -> Result<Option<ProjectAccess>, Error> {
        let project_access = sqlx::query_as!(
            ProjectAccess,
            "SELECT 
                id, 
                project_id, 
                service_account_id, 
                environment_id, 
                enabled, 
                created_at, 
                updated_at
            FROM project_access
            WHERE id = $1
            LIMIT 1",
            id,
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(<sqlx::Error as Into<Error>>::into)?;

        if project_access.is_none() {
            return Err(Error::msg("Project access not found"));
        }

        Ok(project_access)
    }

    async fn update(&self, id: Uuid, update: Self::UpdatePayload) -> Result<ProjectAccess, Error> {
        let mut changes = Vec::new();

        if let Some(enabled) = update.enabled {
            match enabled {
                true => changes.push(("enabled = true", "".to_string())),
                false => changes.push(("enabled = false", "".to_string())),
            }
        }

        if changes.is_empty() {
            return Err(Error::msg("No changes to update"));
        }

        let mut query = QueryBuilder::new("UPDATE project_access SET ");

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
        query
            .push(" RETURNING id, project_id, service_account_id, environment_id, enabled, created_at, updated_at");

        let result = query
            .build()
            .fetch_one(&*self.pool)
            .await
            .map(|row| ProjectAccess {
                id: row.get("id"),
                project_id: row.get("project_id"),
                service_account_id: row.get("service_account_id"),
                environment_id: row.get("environment_id"),
                enabled: row.get("enabled"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });

        match result {
            Ok(project_scope) => Ok(project_scope),
            Err(error) => {
                match error {
                    sqlx::Error::RowNotFound => Err(Error::msg("Project access not found")),
                    sqlx::Error::Database(e) => {
                        let error_message = e.message();

                        match error_message {
                        s if s.contains("unique constraint") || s.contains("duplicate key") => {
                             if s.contains("idx_project_access_project_id_service_account_id_environment_id") {
                                Err(Error::msg("Project Id, Service Account Id and Environment Id combination already exists"))
                            } else {
                                Err(Error::msg("No changes were made"))
                            }
                        }
                        s if s.contains("foreign key") =>
                        {
                            if s.contains("project_access_project_id_fkey") {
                                return Err(Error::msg("Project not found"))
                            }

                            if s.contains("project_access_service_account_id_fkey") {
                                return Err(Error::msg("Service Account not found"))
                            }

                            if s.contains("project_access_environment_id_fkey") {
                                return Err(Error::msg("Environment not found"))
                            }
                            Err(Error::msg("No changes were made"))
                        }
                        _ => Err(Error::msg("No changes were made")),
                    }
                    }
                    _ => Err(error.into()),
                }
            }
        }
    }

    async fn delete(&self, id: Uuid) -> Result<bool, Error> {
        let deleted = sqlx::query!("DELETE FROM project_access WHERE id = $1 RETURNING id", id,)
            .fetch_optional(&*self.pool)
            .await
            .map_err(<sqlx::Error as Into<Error>>::into)?;

        if deleted.is_none() {
            return Err(Error::msg("Project access not found"));
        }

        Ok(true)
    }

    async fn find(
        &self,
        filter: Self::Filter,
        sort: Option<Vec<Self::Sort>>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<ProjectAccess>, Error> {
        let mut query = QueryBuilder::new(
            "SELECT id, project_id, service_account_id, environment_id, enabled, created_at, updated_at FROM project_access ",
        );

        let mut conditions_list: Vec<(&str, String)> = Vec::new();

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

        if let Some(project_id) = &filter.project_id {
            if conditions_list.is_empty() {
                query.push("WHERE ");
            } else {
                query.push(" AND ");
            }
            let project_id = uuid::Uuid::parse_str(&project_id).unwrap();

            query.push(" project_id = ").push_bind(project_id);
        }

        if let Some(service_account_id) = &filter.service_account_id {
            if conditions_list.is_empty() && filter.project_id.is_none() {
                query.push("WHERE ");
            } else {
                query.push(" AND ");
            }
            let service_account_id = uuid::Uuid::parse_str(&service_account_id).unwrap();

            query
                .push(" service_account_id = ")
                .push_bind(service_account_id);
        }

        if let Some(environment_id) = &filter.environment_id {
            if conditions_list.is_empty()
                && filter.project_id.is_none()
                && filter.service_account_id.is_none()
            {
                query.push("WHERE ");
            } else {
                query.push(" AND ");
            }
            let environment_id = uuid::Uuid::parse_str(&environment_id).unwrap();

            query.push(" environment_id = ").push_bind(environment_id);
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

        let project_accesses = query
            .build()
            .fetch_all(&*self.pool)
            .await
            .map_err(<sqlx::Error as Into<Error>>::into)?
            .into_iter()
            .map(|row| ProjectAccess {
                id: row.get("id"),
                project_id: row.get("project_id"),
                service_account_id: row.get("service_account_id"),
                environment_id: row.get("environment_id"),
                enabled: row.get("enabled"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(project_accesses)
    }
}
