use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::{QueryBuilder, Row};
use uuid::Uuid;

use crate::{
    models::{
        pagination::Pagination,
        project_access_scopes::{
            ProjectAccessScope, ProjectAccessScopeCreatePayload, ProjectAccessScopeFilter, ProjectAccessScopeSortOrder,
            ProjectAccessScopeUpdatePayload,
        },
    },
    repositories::base::Repository,
};

#[derive(Clone)]
pub struct ProjectAccessScopesRepository {
    pub pool: Arc<sqlx::postgres::PgPool>,
}

impl ProjectAccessScopesRepository {
    pub fn new(pool: Arc<sqlx::postgres::PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository<ProjectAccessScope> for ProjectAccessScopesRepository {
    type CreatePayload = ProjectAccessScopeCreatePayload;
    type UpdatePayload = ProjectAccessScopeUpdatePayload;
    type Filter = ProjectAccessScopeFilter;
    type Sort = ProjectAccessScopeSortOrder;

    async fn create(&self, item: Self::CreatePayload) -> Result<ProjectAccessScope, Error> {
        let project_access_scope = ProjectAccessScope {
            id: None,
            project_access_id: item.project_access_id.parse().unwrap(),
            scope_id: item.scope_id.parse().unwrap(),
            enabled: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let created = sqlx::query_as!(
            ProjectAccessScope,
            "INSERT INTO project_access_scopes (project_access_id, scope_id, enabled) VALUES ($1, $2, $3) RETURNING id, project_access_id, scope_id, enabled, created_at, updated_at",
            project_access_scope.project_access_id,
            project_access_scope.scope_id,
            project_access_scope.enabled,
        )
        .fetch_one(&*self.pool)
        .await;

        match created {
            Ok(scope) => Ok(scope),
            Err(error) => match error {
                sqlx::Error::RowNotFound => Err(Error::msg("Project access scope not found")),
                sqlx::Error::Database(e) => {
                    let error_message = e.message();
                    match error_message {
                        s if s.contains("unique constraint") || s.contains("duplicate key") => {
                            if s.contains("idx_project_access_scopes_project_access_id_scope_id") {
                                Err(Error::msg("Project Access Id and Scope Id combination already exists"))
                            } else {
                                Err(Error::msg("No changes were made"))
                            }
                        }
                        s if s.contains("foreign key") => Err(Error::msg("Foreign key constraint failed")),
                        _ => Err(Error::msg("No changes were made")),
                    }
                }
                _ => Err(error.into()),
            },
        }
    }

    async fn read(&self, id: Uuid) -> Result<Option<ProjectAccessScope>, Error> {
        let scope = sqlx::query_as!(
            ProjectAccessScope,
            "SELECT id, project_access_id, scope_id, enabled, created_at, updated_at FROM project_access_scopes WHERE id = $1 LIMIT 1",
            id,
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(<sqlx::Error as Into<Error>>::into)?;

        if scope.is_none() {
            return Err(Error::msg("Project access scope not found"));
        }

        Ok(scope)
    }

    async fn update(&self, id: Uuid, update: Self::UpdatePayload) -> Result<ProjectAccessScope, Error> {
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

        let mut query = QueryBuilder::new("UPDATE project_access_scopes SET ");
        let mut separated = query.separated(", ");
        for (field, value) in changes {
            if value.is_empty() {
                separated.push(field);
            } else {
                separated.push(format!("{} = ", field)).push_bind_unseparated(value);
            }
        }
        query.push(", updated_at = ").push_bind(Utc::now());
        query.push(" WHERE id = ").push_bind(id);
        query.push(" RETURNING id, project_access_id, scope_id, enabled, created_at, updated_at");

        let result = query
            .build()
            .fetch_one(&*self.pool)
            .await
            .map(|row| ProjectAccessScope {
                id: row.get("id"),
                project_access_id: row.get("project_access_id"),
                scope_id: row.get("scope_id"),
                enabled: row.get("enabled"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });

        match result {
            Ok(scope) => Ok(scope),
            Err(error) => match error {
                sqlx::Error::RowNotFound => Err(Error::msg("Project access scope not found")),
                sqlx::Error::Database(e) => {
                    let error_message = e.message();
                    match error_message {
                        s if s.contains("unique constraint") || s.contains("duplicate key") => {
                            if s.contains("idx_project_access_scopes_project_access_id_scope_id") {
                                Err(Error::msg("Project Access Id and Scope Id combination already exists"))
                            } else {
                                Err(Error::msg("No changes were made"))
                            }
                        }
                        s if s.contains("foreign key") => Err(Error::msg("Foreign key constraint failed")),
                        _ => Err(Error::msg("No changes were made")),
                    }
                }
                _ => Err(error.into()),
            },
        }
    }

    async fn delete(&self, id: Uuid) -> Result<bool, Error> {
        let deleted = sqlx::query!("DELETE FROM project_access_scopes WHERE id = $1 RETURNING id", id,)
            .fetch_optional(&*self.pool)
            .await
            .map_err(<sqlx::Error as Into<Error>>::into)?;

        if deleted.is_none() {
            return Err(Error::msg("Project access scope not found"));
        }

        Ok(true)
    }

    async fn find(
        &self,
        filter: Self::Filter,
        sort: Option<Vec<Self::Sort>>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<ProjectAccessScope>, Error> {
        let mut query = QueryBuilder::new(
            "SELECT id, project_access_id, scope_id, enabled, created_at, updated_at FROM project_access_scopes ",
        );

        let mut conditions_list = Vec::new();

        if let Some(project_access_id) = filter.project_access_id {
            let uuid = Uuid::parse_str(&project_access_id).unwrap();
            conditions_list.push(("project_access_id = ", uuid));
        }
        if let Some(scope_id) = filter.scope_id {
            let uuid = Uuid::parse_str(&scope_id).unwrap();
            conditions_list.push(("scope_id = ", uuid));
        }

        if !conditions_list.is_empty() {
            query.push("WHERE ");
            let mut conditions = query.separated(" AND ");
            for (condition, value) in &conditions_list {
                conditions.push(condition).push_bind_unseparated(value);
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

        let scopes = query
            .build()
            .fetch_all(&*self.pool)
            .await
            .map_err(<sqlx::Error as Into<Error>>::into)?
            .into_iter()
            .map(|row| ProjectAccessScope {
                id: row.get("id"),
                project_access_id: row.get("project_access_id"),
                scope_id: row.get("scope_id"),
                enabled: row.get("enabled"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(scopes)
    }
} 