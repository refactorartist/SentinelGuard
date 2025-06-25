use crate::models::pagination::Pagination;
use crate::models::project_access_scopes::{
    ProjectAccessScope, ProjectAccessScopeCreatePayload, ProjectAccessScopeFilter,
    ProjectAccessScopeSortOrder, ProjectAccessScopeUpdatePayload,
};
use crate::repositories::base::Repository;
use crate::repositories::project_access_scopes_repository::ProjectAccessScopesRepository;
use crate::services::base::Service;
use anyhow::Error;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone)]
pub struct ProjectAccessScopesService {
    pub repository: ProjectAccessScopesRepository,
}

impl ProjectAccessScopesService {
    pub fn new(repo: ProjectAccessScopesRepository) -> Self {
        Self { repository: repo }
    }
}

#[async_trait]
impl Service<ProjectAccessScope> for ProjectAccessScopesService {
    type CreatePayload = ProjectAccessScopeCreatePayload;
    type UpdatePayload = ProjectAccessScopeUpdatePayload;
    type Filter = ProjectAccessScopeFilter;
    type Sort = ProjectAccessScopeSortOrder;

    async fn create(&self, item: Self::CreatePayload) -> Result<ProjectAccessScope, Error> {
        self.repository.create(item).await
    }

    async fn read(&self, id: Uuid) -> Result<Option<ProjectAccessScope>, Error> {
        self.repository.read(id).await
    }

    async fn update(&self, id: Uuid, update: Self::UpdatePayload) -> Result<ProjectAccessScope, Error> {
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
    ) -> Result<Vec<ProjectAccessScope>, Error> {
        self.repository.find(filter, sort, pagination).await
    }
}
