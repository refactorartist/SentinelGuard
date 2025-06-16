use crate::models::pagination::Pagination;
use anyhow::Error;
use async_trait::async_trait;
use serde::Serialize;
use serde::de::DeserializeOwned;
use uuid::Uuid;

#[async_trait]
pub trait Service<T: Send + Sync + Serialize + DeserializeOwned + 'static> {
    type CreatePayload: Send + Sync + Serialize + DeserializeOwned + 'static;
    type UpdatePayload: Send + Sync + Serialize + DeserializeOwned + 'static;
    type Filter: Send + Sync + Serialize + DeserializeOwned + 'static;
    type Sort: Send + Sync + Serialize + DeserializeOwned + 'static;

    async fn create(&self, item: Self::CreatePayload) -> Result<T, Error>;

    async fn read(&self, id: Uuid) -> Result<Option<T>, Error>;

    async fn update(&self, id: Uuid, update: Self::UpdatePayload) -> Result<T, Error>;

    async fn delete(&self, id: Uuid) -> Result<bool, Error>;

    async fn find(
        &self,
        filter: Self::Filter,
        sort: Option<Vec<Self::Sort>>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<T>, Error>;
}
