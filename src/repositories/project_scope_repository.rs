use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Row, QueryBuilder};
use uuid::Uuid;

use crate::{
    models::{
        pagination::Pagination,
        project_scope::{
            ProjectScope, ProjectScopeCreatePayload, ProjectScopeFilter, ProjectScopeSortOrder,
            ProjectScopeUpdatePayload,
        },
    },
    repositories::base::Repository,
};

#[derive(Clone)]
pub struct ProjectScopeRepository {
    pub pool: Arc<sqlx::postgres::PgPool>,
}

impl ProjectScopeRepository {
    pub fn new(pool: Arc<sqlx::postgres::PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository<ProjectScope> for ProjectScopeRepository {
    type CreatePayload = ProjectScopeCreatePayload;
    type UpdatePayload = ProjectScopeUpdatePayload;
    type Filter = ProjectScopeFilter;
    type Sort = ProjectScopeSortOrder;

    async fn create(&self, item: Self::CreatePayload) -> Result<ProjectScope, Error> {
        let project_scope = ProjectScope {
            id: None,
            project_id: item.project_id.parse().unwrap(),
            scope: item.scope,
            description: item.description,
            enabled: item.enabled,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let created_project_scope = sqlx::query_as!(
            ProjectScope,
            "INSERT INTO project_scopes (project_id, scope, description, enabled) VALUES ($1, $2, $3, $4) RETURNING id, project_id, scope, description, enabled, created_at, updated_at",
            project_scope.project_id,
            project_scope.scope,
            project_scope.description,
            project_scope.enabled,
        )
        .fetch_one(&*self.pool)
        .await;

        match created_project_scope {
            Ok(project_scope) => Ok(project_scope),
            Err(error) => match error {
                sqlx::Error::RowNotFound => Err(Error::msg("Project scope not found")),
                sqlx::Error::Database(e) => {
                    let error_message = e.message();

                    match error_message {
                        s if s.contains("unique constraint") || s.contains("duplicate key") => {
                            if s.contains("idx_project_scopes_project_id_scope") {
                                Err(Error::msg("Project Id, scope combination already exists"))
                            } else {
                                Err(Error::msg("No changes were made"))
                            }
                        }
                        s if s.contains("foreign key")
                            && s.contains("project_scopes_project_id_fkey") =>
                        {
                            Err(Error::msg("Project not found"))
                        }
                        _ => Err(Error::msg("No changes were made")),
                    }
                }
                _ => Err(error.into()),
            },
        }
    }

    async fn read(&self, id: Uuid) -> Result<Option<ProjectScope>, Error> {
        let project_scope = sqlx::query_as!(
            ProjectScope,
            "SELECT * FROM project_scopes WHERE id = $1 LIMIT 1",
            id,
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(<sqlx::Error as Into<Error>>::into)?;

        if project_scope.is_none() {
            return Err(Error::msg("Project scope not found"));
        }

        Ok(project_scope)
    }

    async fn update(&self, id: Uuid, update: Self::UpdatePayload) -> Result<ProjectScope, Error> {
        let mut changes = Vec::new(); 

        if let Some(scope) = update.scope {
            changes.push(("scope", scope));
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

        let mut query = QueryBuilder::new("UPDATE project_scopes SET ");

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
        query.push(" RETURNING id, project_id, scope, description, enabled, created_at, updated_at");

        let result = query
            .build()
            .fetch_one(&*self.pool)
            .await
            .map(|row| ProjectScope {
                id: row.get("id"),
                project_id: row.get("project_id"),
                scope: row.get("scope"),
                description: row.get("description"),
                enabled: row.get("enabled"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });

        match result {
            Ok(project_scope) => Ok(project_scope),
            Err(error) => match error {
                sqlx::Error::RowNotFound => Err(Error::msg("Project scope not found")),
                sqlx::Error::Database(e) => {
                    let error_message = e.message();

                    match error_message {
                        s if s.contains("unique constraint") || s.contains("duplicate key") => {
                            if s.contains("idx_project_scopes_project_id_scope") {
                                Err(Error::msg("Project Id, scope combination already exists"))
                            } else {
                                Err(Error::msg("No changes were made"))
                            }
                        }
                        s if s.contains("foreign key")
                            && s.contains("project_scopes_project_id_fkey") =>
                        {
                            Err(Error::msg("Project not found"))
                        }
                        _ => Err(Error::msg("No changes were made")),
                    }
                }
                _ => Err(error.into()),
            },
        }
    }

    async fn delete(&self, _id: Uuid) -> Result<bool, Error> {
        todo!()
    }

    async fn find(
        &self,
        _filter: Self::Filter,
        _sort: Option<Vec<Self::Sort>>,
        _pagination: Option<Pagination>,
    ) -> Result<Vec<ProjectScope>, Error> {
        todo!()
    }
}
