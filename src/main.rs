use actix_web::{HttpServer, web};
use sentinel_guard::config::AppConfig;
use sentinel_guard::repositories::environment_key_repository::EnvironmentKeyRepository;
use sentinel_guard::repositories::environment_repository::EnvironmentRepository;
use sentinel_guard::repositories::project_access_repository::ProjectAccessRepository;
use sentinel_guard::repositories::project_access_scopes_repository::ProjectAccessScopesRepository;
use sentinel_guard::repositories::project_repository::ProjectRepository;
use sentinel_guard::repositories::project_scope_repository::ProjectScopeRepository;
use sentinel_guard::repositories::service_account_repository::ServiceAccountRepository;
use sentinel_guard::routes::{
    environment_route, project_access_route, project_access_scopes_route, project_route,
    project_scope_route, service_account_route,
};
use sentinel_guard::services::environment_key_service::EnvironmentKeyService;
use sentinel_guard::services::environment_service::EnvironmentService;
use sentinel_guard::services::project_access_scopes_service::ProjectAccessScopesService;
use sentinel_guard::services::project_access_service::ProjectAccessService;
use sentinel_guard::services::project_scope_service::ProjectScopeService;
use sentinel_guard::services::project_service::ProjectService;
use sentinel_guard::services::service_account_service::ServiceAccountService;
use sentinel_guard::utils::swagger::get_swagger_ui;
use sqlx::postgres::PgPool;
use std::{sync::Arc, time::Duration};
use tokio::signal;

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = AppConfig::from_env(Some(true))?;

    let pool = Arc::new(PgPool::connect(&config.database_uri).await?);
    let host = config.host;
    let port = config.port;

    let server = HttpServer::new(move || {
        let app = actix_web::App::new()
            .app_data(web::Data::new(ProjectService::new(ProjectRepository::new(
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
            )));

        let routes = [
            project_route::configure_routes,
            service_account_route::configure_routes,
            project_scope_route::configure_routes,
            environment_route::configure_routes,
            project_access_route::configure_routes,
            project_access_scopes_route::configure_routes,
        ];

        app.configure(|config| {
            for route in routes {
                route(config);
            }
        })
        .service(get_swagger_ui())
    })
    .bind((host.clone(), port))?
    .shutdown_timeout(30) // 30 seconds graceful shutdown timeout
    .workers(4) // Set number of workers
    .keep_alive(Duration::from_secs(75)) // Keep-alive timeout
    .run();

    println!("Server started at http://{}:{}", host, port);

    let server_handle = server.handle();

    // Wait for the server to finish or for a Ctrl+C signal
    tokio::select! {
        _ = server => {
            println!("Server finished");
        }
        _ = signal::ctrl_c() => {
            println!("Received ctrl+c signal, shutting down gracefully");

            // Stop accepting new connections
            server_handle.stop(true).await;

            // Give some time for cleanup
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("Cleanup completed, server shutting down");
        }
    }

    Ok(())
}
