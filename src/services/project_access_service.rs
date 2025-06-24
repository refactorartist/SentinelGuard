use crate::models::pagination::Pagination;
use crate::models::project_access::{
    ProjectAccess, ProjectAccessCreatePayload, ProjectAccessFilter, ProjectAccessSortOrder,
    ProjectAccessUpdatePayload,
};
use crate::repositories::base::Repository;
use crate::repositories::project_access_repository::ProjectAccessRepository;
use crate::services::base::Service;
use anyhow::Error;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone)]
pub struct ProjectAccessService {
    pub repository: ProjectAccessRepository,
}

impl ProjectAccessService {
    pub fn new(repo: ProjectAccessRepository) -> Self {
        Self { repository: repo }
    }
}

#[async_trait]
impl Service<ProjectAccess> for ProjectAccessService {
    type CreatePayload = ProjectAccessCreatePayload;
    type UpdatePayload = ProjectAccessUpdatePayload;
    type Filter = ProjectAccessFilter;
    type Sort = ProjectAccessSortOrder;

    async fn create(&self, item: Self::CreatePayload) -> Result<ProjectAccess, Error> {
        self.repository.create(item).await
    }

    async fn read(&self, id: Uuid) -> Result<Option<ProjectAccess>, Error> {
        self.repository.read(id).await
    }

    async fn update(&self, id: Uuid, update: Self::UpdatePayload) -> Result<ProjectAccess, Error> {
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
    ) -> Result<Vec<ProjectAccess>, Error> {
        self.repository.find(filter, sort, pagination).await
    }
}
