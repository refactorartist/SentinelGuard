use std::sync::Arc;

use sqlx::PgPool;

use sentinel_guard::{
    models::project::{Project, ProjectCreatePayload},
    repositories::project_repository::ProjectRepository,
    routes::project_route,
    services::project_service::ProjectService,
};

include!("../../commons/macros.rs");

fn services(pool: PgPool) -> Vec<ProjectService> {
    vec![ProjectService::new(ProjectRepository::new(Arc::new(pool)))]
}

fn routes() -> Vec<fn(&mut actix_web::web::ServiceConfig)> {
    vec![project_route::configure_routes]
}

#[sqlx::test]
async fn test_project_route_create_project_with_valid_data_succeeds(pool: PgPool) {
    let app = create_test_app!(
        services(pool),
        routes()
    );

    let project = ProjectCreatePayload {
        name: "test".to_string(),
        description: "test".to_string(),
        enabled: true,
    };

    let response = actix_web::test::TestRequest::post()
        .uri("/projects")
        .set_json(&project)
        .send_request(&app)
        .await;

    assert!(response.status().is_success());

    let created_project: Project = actix_web::test::read_body_json(response).await;

    assert_eq!(created_project.name, "test");
    assert_eq!(created_project.description, "test");
    assert!(created_project.enabled);
}

#[sqlx::test(fixtures("../fixtures/projects.sql"))]
async fn test_project_route_create_project_with_duplicate_name_fails(pool: PgPool) {
    let app = create_test_app!(
        services(pool),
        routes()
    );

    let project = ProjectCreatePayload {
        name: "testa".to_string(),
        description: "test".to_string(),
        enabled: true,
    };

    let response = actix_web::test::TestRequest::post()
        .uri("/projects")
        .set_json(&project)
        .send_request(&app)
        .await;

    assert!(response.status().is_client_error());

    assert_eq!(response.status(), actix_web::http::StatusCode::BAD_REQUEST);

}


