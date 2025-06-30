use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::{App, web};
use sqlx::postgres::PgPool;
use std::sync::Arc;

use crate::repositories::{
    environment_key_repository::EnvironmentKeyRepository,
    environment_repository::EnvironmentRepository,
    project_access_repository::ProjectAccessRepository,
    project_access_scopes_repository::ProjectAccessScopesRepository,
    project_repository::ProjectRepository, project_scope_repository::ProjectScopeRepository,
    service_account_repository::ServiceAccountRepository,
};
use crate::services::{
    environment_key_service::EnvironmentKeyService, environment_service::EnvironmentService,
    project_access_scopes_service::ProjectAccessScopesService,
    project_access_service::ProjectAccessService, project_scope_service::ProjectScopeService,
    project_service::ProjectService, service_account_service::ServiceAccountService,
};

pub fn register_services<T>(app: App<T>, pool: Arc<PgPool>) -> App<T>
where
    T: ServiceFactory<
            ServiceRequest,
            Config = (),
            Response = ServiceResponse,
            Error = actix_web::Error,
            InitError = (),
        >,
{
    app.app_data(web::Data::new(ProjectService::new(ProjectRepository::new(
        pool.clone(),
    ))))
    .app_data(web::Data::new(ServiceAccountService::new(
        ServiceAccountRepository::new(pool.clone()),
    )))
    .app_data(web::Data::new(ProjectScopeService::new(
        ProjectScopeRepository::new(pool.clone()),
    )))
    .app_data(web::Data::new(EnvironmentService::new(
        EnvironmentRepository::new(pool.clone()),
    )))
    .app_data(web::Data::new(ProjectAccessService::new(
        ProjectAccessRepository::new(pool.clone()),
    )))
    .app_data(web::Data::new(ProjectAccessScopesService::new(
        ProjectAccessScopesRepository::new(pool.clone()),
    )))
    .app_data(web::Data::new(EnvironmentKeyService::new(
        EnvironmentKeyRepository::new(pool.clone()),
    )))
}
