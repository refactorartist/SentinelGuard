use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use sqlx;
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
        todo!()
    }

    async fn read(&self, id: Uuid) -> Result<Option<ProjectScope>, Error> {
        todo!()
    }

    async fn update(&self, id: Uuid, update: Self::UpdatePayload) -> Result<ProjectScope, Error> {
        todo!()
    }

    async fn delete(&self, id: Uuid) -> Result<bool, Error> {
        todo!()
    }

    async fn find(
        &self,
        filter: Self::Filter,
        sort: Option<Vec<Self::Sort>>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<ProjectScope>, Error> {
        todo!()
    }
}
