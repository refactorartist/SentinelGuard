use crate::models::pagination::Pagination;
use crate::models::project_scope::{
    ProjectScope, ProjectScopeCreatePayload, ProjectScopeFilter, ProjectScopeSortOrder,
    ProjectScopeUpdatePayload,
};
use crate::repositories::base::Repository;
use crate::repositories::project_scope_repository::ProjectScopeRepository;
use crate::services::base::Service;
use anyhow::Error;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone)]
pub struct ProjectScopeService {
    pub repository: ProjectScopeRepository,
}

impl ProjectScopeService {
    pub fn new(repo: ProjectScopeRepository) -> Self {
        Self { repository: repo }
    }
}

#[async_trait]
impl Service<ProjectScope> for ProjectScopeService {
    type CreatePayload = ProjectScopeCreatePayload;
    type UpdatePayload = ProjectScopeUpdatePayload;
    type Filter = ProjectScopeFilter;
    type Sort = ProjectScopeSortOrder;

    async fn create(&self, item: Self::CreatePayload) -> Result<ProjectScope, Error> {
        self.repository.create(item).await
    }

    async fn read(&self, id: Uuid) -> Result<Option<ProjectScope>, Error> {
        self.repository.read(id).await
    }

    async fn update(&self, id: Uuid, update: Self::UpdatePayload) -> Result<ProjectScope, Error> {
        self.repository.update(id, update).await
    }

    async fn delete(&self, id: Uuid) -> Result<bool, Error> {
        self.repository.delete(id).await
    }

    async fn find(
        &self,
        filter: Self::Filter,
        sort: Option<Vec<Self::Sort>>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<ProjectScope>, Error> {
        self.repository.find(filter, sort, pagination).await
    }
}
