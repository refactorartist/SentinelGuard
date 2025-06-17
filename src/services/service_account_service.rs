use crate::{
    models::{
        pagination::Pagination,
        service_account::{
            ServiceAccount, ServiceAccountCreatePayload, ServiceAccountFilter,
            ServiceAccountSortOrder, ServiceAccountUpdatePayload,
        },
    },
    repositories::{
        service_account_repository::ServiceAccountRepository,
        base::Repository,
    },
    services::base::Service,
};
use async_trait::async_trait;
use uuid::Uuid;
use anyhow::Error;

#[derive(Clone)]
pub struct ServiceAccountService {
    pub repository: ServiceAccountRepository
}

impl ServiceAccountService {
    pub fn new (repo: ServiceAccountRepository) -> Self {
        return Self{
            repository: repo
        }
    }
}

#[async_trait]
impl Service<ServiceAccount> for ServiceAccountService {
    type CreatePayload = ServiceAccountCreatePayload;
    type UpdatePayload = ServiceAccountUpdatePayload;
    type Filter = ServiceAccountFilter;
    type Sort = ServiceAccountSortOrder;
    

    async fn create(&self, item: Self::CreatePayload) -> Result<ServiceAccount, Error> {
        self.repository.create(item).await
    }

    async fn read(&self, id: Uuid) -> Result<Option<ServiceAccount>, Error> {
        self.repository.read(id).await
    }

    async fn update(&self, id: Uuid, update: Self::UpdatePayload) -> Result<ServiceAccount, Error> {
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
    ) -> Result<Vec<ServiceAccount>, Error> {
        self.repository.find(filter, sort, pagination).await
    }    
    
}
