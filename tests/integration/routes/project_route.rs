use std::sync::Arc;

use sqlx::PgPool;

use sentinel_guard::{
    models::project::{Project, ProjectCreatePayload, ProjectUpdatePayload},
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
    let app = create_test_app!(services(pool), routes());

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
    let app = create_test_app!(services(pool), routes());

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

#[sqlx::test(fixtures("../fixtures/projects.sql"))]
async fn test_project_route_get_project_by_id_succeeds(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let response = actix_web::test::TestRequest::get()
        .uri("/projects/123e4567-e89b-12d3-a456-426614174000")
        .send_request(&app)
        .await;

    assert!(response.status().is_success());

    let project: Project = actix_web::test::read_body_json(response).await;

    assert_eq!(project.name, "testa");
    assert_eq!(project.description, "test");
    assert!(project.enabled);
}

#[sqlx::test]
async fn test_project_route_get_project_by_id_not_found(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let response = actix_web::test::TestRequest::get()
        .uri("/projects/123e4567-e89b-12d3-a456-426614174000")
        .send_request(&app)
        .await;

    assert!(response.status().is_client_error());

    assert_eq!(response.status(), actix_web::http::StatusCode::NOT_FOUND);
}

#[sqlx::test(fixtures("../fixtures/projects.sql"))]
async fn test_project_route_patch_project_name_succeeds(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let payload = ProjectUpdatePayload {
        name: Some("testc".to_string()),
        description: None,
        enabled: None,
    };

    let response = actix_web::test::TestRequest::patch()
        .uri("/projects/123e4567-e89b-12d3-a456-426614174000")
        .set_json(&payload)
        .send_request(&app)
        .await;

    assert!(response.status().is_success());
}

#[sqlx::test(fixtures("../fixtures/projects.sql"))]
async fn test_project_route_patch_project_description_succeeds(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let payload = ProjectUpdatePayload {
        name: None,
        description: Some("something to change immediately".to_string()),
        enabled: None,
    };

    let response = actix_web::test::TestRequest::patch()
        .uri("/projects/123e4567-e89b-12d3-a456-426614174001")
        .set_json(&payload)
        .send_request(&app)
        .await;

    let (_request, response) = response.into_parts();

    let body = response.body();

    dbg!(&body);

    assert!(response.status().is_success());
}

#[sqlx::test(fixtures("../fixtures/projects.sql"))]
async fn test_project_route_patch_project_enabled_succeeds(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let payload = ProjectUpdatePayload {
        name: None,
        description: None,
        enabled: Some(false),
    };

    let response = actix_web::test::TestRequest::patch()
        .uri("/projects/123e4567-e89b-12d3-a456-426614174000")
        .set_json(&payload)
        .send_request(&app)
        .await;

    assert!(response.status().is_success());
}


#[sqlx::test(fixtures("../fixtures/projects.sql"))]
async fn test_project_route_patch_project_empty_payload(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let payload = ProjectUpdatePayload {
        name: None,
        description: None,
        enabled: None,
    };

    let response = actix_web::test::TestRequest::patch()
        .uri("/projects/123e4567-e89b-12d3-a456-426614174000")
        .set_json(&payload)
        .send_request(&app)
        .await;

    assert!(response.status().is_client_error());
    assert_eq!(response.status(), actix_web::http::StatusCode::BAD_REQUEST);
}


#[sqlx::test]
async fn test_project_route_patch_project_not_found(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let payload = ProjectUpdatePayload {
        name: Some("test".to_string()),
        description: None,
        enabled: None,
    };

    let response = actix_web::test::TestRequest::patch()
        .uri("/projects/123e4567-e89b-12d3-a456-426614178000")
        .set_json(&payload)
        .send_request(&app)
        .await;

    assert!(response.status().is_client_error());
    assert_eq!(response.status(), actix_web::http::StatusCode::NOT_FOUND);
}
