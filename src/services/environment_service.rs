use crate::models::environment::{
    Environment, EnvironmentCreatePayload, EnvironmentFilter, EnvironmentSortOrder,
    EnvironmentUpdatePayload,
};
use crate::models::pagination::Pagination;
use crate::repositories::base::Repository;
use crate::repositories::environment_repository::EnvironmentRepository;
use crate::services::base::Service;
use anyhow::Error;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone)]
pub struct EnvironmentService {
    pub repository: EnvironmentRepository,
}

impl EnvironmentService {
    pub fn new(repo: EnvironmentRepository) -> Self {
        Self { repository: repo }
    }
}

#[async_trait]
impl Service<Environment> for EnvironmentService {
    type CreatePayload = EnvironmentCreatePayload;
    type UpdatePayload = EnvironmentUpdatePayload;
    type Filter = EnvironmentFilter;
    type Sort = EnvironmentSortOrder;

    async fn create(&self, item: Self::CreatePayload) -> Result<Environment, Error> {
        self.repository.create(item).await
    }

    async fn read(&self, id: Uuid) -> Result<Option<Environment>, Error> {
        self.repository.read(id).await
    }

    async fn update(&self, id: Uuid, update: Self::UpdatePayload) -> Result<Environment, Error> {
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
    ) -> Result<Vec<Environment>, Error> {
        self.repository.find(filter, sort, pagination).await
    }
}
