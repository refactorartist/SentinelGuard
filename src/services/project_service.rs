use crate::models::pagination::Pagination;
use crate::models::project::{
    Project, ProjectCreatePayload, ProjectFilter, ProjectSortOrder, ProjectUpdatePayload,
};
use crate::repositories::base::Repository;
use crate::repositories::project_repository::ProjectRepository;
use crate::services::base::Service;
use anyhow::Error;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone)]
pub struct ProjectService {
    pub repository: ProjectRepository,
}

impl ProjectService {
    pub fn new(repo: ProjectRepository) -> Self {
        Self { repository: repo }
    }
}

#[async_trait]
impl Service<Project> for ProjectService {
    type CreatePayload = ProjectCreatePayload;
    type UpdatePayload = ProjectUpdatePayload;
    type Filter = ProjectFilter;
    type Sort = ProjectSortOrder;

    async fn create(&self, item: Self::CreatePayload) -> Result<Project, Error> {
        self.repository.create(item).await
    }

    async fn read(&self, id: Uuid) -> Result<Option<Project>, Error> {
        self.repository.read(id).await
    }

    async fn update(&self, id: Uuid, update: Self::UpdatePayload) -> Result<Project, Error> {
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
    ) -> Result<Vec<Project>, Error> {
        self.repository.find(filter, sort, pagination).await
    }
}
