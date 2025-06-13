use sentinel_guard::{config::AppConfig, routes::project_route};
use sentinel_guard::repositories::project_repository::ProjectRepository;
use sentinel_guard::services::project_service::ProjectService;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPool;
use tokio::signal;
use std::{sync::Arc, time::Duration};

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = AppConfig::from_env(Some(true))?;
    
    let pool = Arc::new(PgPool::connect(&config.database_uri).await?);
    

    let project_service = ProjectService::new(ProjectRepository::new(pool.clone()));
    
    let host = config.host;
    let port = config.port;
    
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(project_service.clone()))
            .configure(project_route::configure_routes)
            
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