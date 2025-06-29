use crate::models::environment_key::{EnvironmentKey, EnvironmentKeyCreatePayload, EnvironmentKeyFilter, EnvironmentKeySortOrder, EnvironmentKeyUpdatePayload};
use crate::models::pagination::Pagination;
use crate::repositories::base::Repository;
use crate::repositories::environment_key_repository::EnvironmentKeyRepository;
use crate::services::base::Service;
use anyhow::Error;
use async_trait::async_trait;
use jsonwebtoken::Algorithm;
use uuid::Uuid;

#[derive(Clone)]
pub struct EnvironmentKeyService {
    pub repository: EnvironmentKeyRepository,
}

impl EnvironmentKeyService {
    pub fn new(repo: EnvironmentKeyRepository) -> Self {
        Self { repository: repo }
    }
}

impl EnvironmentKeyService {
    pub async fn get_environment_key(
        self,
        environment_id: Uuid,
        algorithm: Algorithm,
    ) -> Result<String, Error> {
        let key = self.repository.get_environment_key(environment_id, algorithm).await?;
        Ok(key)
    }
}

#[async_trait]
impl Service<EnvironmentKey> for EnvironmentKeyService {
    type CreatePayload = EnvironmentKeyCreatePayload;
    type UpdatePayload = EnvironmentKeyUpdatePayload;
    type Filter = EnvironmentKeyFilter;
    type Sort = EnvironmentKeySortOrder;

    async fn create(&self, item: Self::CreatePayload) -> Result<EnvironmentKey, Error> {
        self.repository.create(item).await
    }

    async fn read(&self, id: Uuid) -> Result<Option<EnvironmentKey>, Error> {
        self.repository.read(id).await
    }

    async fn update(&self, id: Uuid, update: Self::UpdatePayload) -> Result<EnvironmentKey, Error> {
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
    ) -> Result<Vec<EnvironmentKey>, Error> {
        self.repository.find(filter, sort, pagination).await
    }
}