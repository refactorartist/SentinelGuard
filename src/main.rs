use actix_web::{HttpServer, web};
use sentinel_guard::repositories::project_repository::ProjectRepository;
use sentinel_guard::repositories::project_scope_repository::ProjectScopeRepository;
use sentinel_guard::repositories::service_account_repository::ServiceAccountRepository;
use sentinel_guard::routes::service_account_route;
use sentinel_guard::services::project_scope_service::ProjectScopeService;
use sentinel_guard::services::project_service::ProjectService;
use sentinel_guard::services::service_account_service::ServiceAccountService;
use sentinel_guard::{config::AppConfig, routes::project_route, routes::project_scope_route};
use sqlx::postgres::PgPool;
use std::{sync::Arc, time::Duration};
use tokio::signal;
use utoipa::OpenApi;

use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            project_route::post,
            project_route::get,
            project_route::patch,
            project_route::delete,
            project_route::list,
            service_account_route::post,
            service_account_route::get,
            service_account_route::patch,
            service_account_route::delete,
            service_account_route::list,
            project_scope_route::post,
            project_scope_route::get,
            project_scope_route::patch,
            project_scope_route::delete,
            project_scope_route::list,
        ),
        tags(
            (name = "SentinelGuard", description = "SentinelGuard API documentation.")
        ),
    )]
    struct ApiDoc;

    let config = AppConfig::from_env(Some(true))?;

    let pool = Arc::new(PgPool::connect(&config.database_uri).await?);

    let project_service = ProjectService::new(ProjectRepository::new(pool.clone()));
    let service_account_service =
        ServiceAccountService::new(ServiceAccountRepository::new(pool.clone()));
    let project_scope_service = ProjectScopeService::new(ProjectScopeRepository::new(pool.clone()));

    let host = config.host;
    let port = config.port;
    let server = HttpServer::new(move || {
        actix_web::App::new()
            .app_data(web::Data::new(project_service.clone()))
            .app_data(web::Data::new(service_account_service.clone()))
            .app_data(web::Data::new(project_scope_service.clone()))
            .configure(project_route::configure_routes)
            .configure(service_account_route::configure_routes)
            .service(
                SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
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
