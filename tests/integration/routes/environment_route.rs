use std::sync::Arc;

use sentinel_guard::{
    models::environment::{
        EnvironmentCreatePayload, EnvironmentResponse, EnvironmentUpdatePayload,
    },
    repositories::environment_repository::EnvironmentRepository,
    routes::environment_route,
    services::environment_service::EnvironmentService,
};
use sqlx::PgPool;

use crate::create_test_app;

fn services(pool: PgPool) -> EnvironmentService {
    EnvironmentService::new(EnvironmentRepository::new(Arc::new(pool)))
}

fn routes() -> fn(&mut actix_web::web::ServiceConfig) {
    environment_route::configure_routes
}

#[sqlx::test(fixtures("../fixtures/projects.sql"))]
async fn test_environment_route_create_environment_with_valid_data_succeeds(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let environment = EnvironmentCreatePayload {
        name: "test-env".to_string(),
        description: "Test Environment".to_string(),
        enabled: true,
        project_id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
    };

    let response = actix_web::test::TestRequest::post()
        .uri("/environments")
        .set_json(&environment)
        .send_request(&app)
        .await;

    assert!(response.status().is_success());
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_route_create_environment_with_duplicate_project_id_name_fails(
    pool: PgPool,
) {
    let app = create_test_app!(services(pool), routes());

    let environment = EnvironmentCreatePayload {
        name: "dev".to_string(),
        description: "Development environment".to_string(),
        enabled: true,
        project_id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
    };

    let response = actix_web::test::TestRequest::post()
        .uri("/environments")
        .set_json(&environment)
        .send_request(&app)
        .await;

    assert!(response.status().is_client_error());
    assert_eq!(response.status(), actix_web::http::StatusCode::BAD_REQUEST);
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_route_read_environment_by_id_successful(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let response = actix_web::test::TestRequest::get()
        .uri("/environments/00000000-0000-0000-0000-000000000001")
        .send_request(&app)
        .await;

    assert!(response.status().is_success());
}

#[sqlx::test]
async fn test_environment_route_read_environment_by_id_not_found(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let response = actix_web::test::TestRequest::get()
        .uri("/environments/00000000-0000-0000-0000-000000000099")
        .send_request(&app)
        .await;

    assert!(response.status().is_client_error());
    assert_eq!(response.status(), actix_web::http::StatusCode::NOT_FOUND);
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_route_patch_name_successful(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let payload = EnvironmentUpdatePayload {
        name: Some("updated-name".to_string()),
        description: None,
        enabled: None,
    };

    let response = actix_web::test::TestRequest::patch()
        .uri("/environments/00000000-0000-0000-0000-000000000001")
        .set_json(&payload)
        .send_request(&app)
        .await;

    assert!(response.status().is_success());
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_route_patch_description_successful(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let payload = EnvironmentUpdatePayload {
        name: None,
        description: Some("Updated description".to_string()),
        enabled: None,
    };

    let response = actix_web::test::TestRequest::patch()
        .uri("/environments/00000000-0000-0000-0000-000000000001")
        .set_json(&payload)
        .send_request(&app)
        .await;

    assert!(response.status().is_success());
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_route_patch_enabled_true_successful(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let payload = EnvironmentUpdatePayload {
        name: None,
        description: None,
        enabled: Some(true),
    };

    let response = actix_web::test::TestRequest::patch()
        .uri("/environments/00000000-0000-0000-0000-000000000002")
        .set_json(&payload)
        .send_request(&app)
        .await;

    assert!(response.status().is_success());
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_route_patch_enabled_false_successful(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let payload = EnvironmentUpdatePayload {
        name: None,
        description: None,
        enabled: Some(false),
    };

    let response = actix_web::test::TestRequest::patch()
        .uri("/environments/00000000-0000-0000-0000-000000000001")
        .set_json(&payload)
        .send_request(&app)
        .await;

    assert!(response.status().is_success());
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_route_patch_duplicate_project_id_name_fails(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let payload = EnvironmentUpdatePayload {
        name: Some("staging".to_string()),
        description: None,
        enabled: None,
    };

    let response = actix_web::test::TestRequest::patch()
        .uri("/environments/00000000-0000-0000-0000-000000000001")
        .set_json(&payload)
        .send_request(&app)
        .await;

    assert!(response.status().is_client_error());
    assert_eq!(response.status(), actix_web::http::StatusCode::CONFLICT);
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_route_delete_environment_successful(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let response = actix_web::test::TestRequest::delete()
        .uri("/environments/00000000-0000-0000-0000-000000000001")
        .send_request(&app)
        .await;

    assert_eq!(response.status(), actix_web::http::StatusCode::NO_CONTENT);
}

#[sqlx::test]
async fn test_environment_route_delete_environment_not_found(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let response = actix_web::test::TestRequest::delete()
        .uri("/environments/00000000-0000-0000-0000-000000000099")
        .send_request(&app)
        .await;

    assert!(response.status().is_client_error());
    assert_eq!(response.status(), actix_web::http::StatusCode::NOT_FOUND);
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_route_list_environments_filter_by_project_id(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let response = actix_web::test::TestRequest::get()
        .uri("/environments?project_id=123e4567-e89b-12d3-a456-426614174000")
        .send_request(&app)
        .await;

    let environments: Vec<EnvironmentResponse> = actix_web::test::read_body_json(response).await;
    assert!(!environments.is_empty());
    assert!(environments
        .iter()
        .all(|e| e.project_id == "123e4567-e89b-12d3-a456-426614174000"));
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_route_list_environments_filter_by_enabled_true(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let response = actix_web::test::TestRequest::get()
        .uri("/environments?enabled=true")
        .send_request(&app)
        .await;

    let environments: Vec<EnvironmentResponse> = actix_web::test::read_body_json(response).await;
    assert!(!environments.is_empty());
    assert!(environments.iter().all(|e| e.enabled));
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_route_list_environments_filter_by_enabled_false(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let response = actix_web::test::TestRequest::get()
        .uri("/environments?enabled=false")
        .send_request(&app)
        .await;

    let environments: Vec<EnvironmentResponse> = actix_web::test::read_body_json(response).await;
    assert!(!environments.is_empty());
    assert!(environments.iter().all(|e| !e.enabled));
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_route_list_environments_filter_by_name(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let response = actix_web::test::TestRequest::get()
        .uri("/environments?name=dev")
        .send_request(&app)
        .await;

    let environments: Vec<EnvironmentResponse> = actix_web::test::read_body_json(response).await;
    assert!(!environments.is_empty());
    assert!(environments.iter().all(|e| e.name == "dev"));
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_route_list_environments_filter_by_description(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let response = actix_web::test::TestRequest::get()
        .uri("/environments?description=Development")
        .send_request(&app)
        .await;

    let environments: Vec<EnvironmentResponse> = actix_web::test::read_body_json(response).await;
    assert!(!environments.is_empty());
    assert!(environments
        .iter()
        .all(|e| e.description.contains("Development")));
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_route_list_environments_limit_success(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let response = actix_web::test::TestRequest::get()
        .uri("/environments?limit=1")
        .send_request(&app)
        .await;

    let environments: Vec<EnvironmentResponse> = actix_web::test::read_body_json(response).await;
    assert!(!environments.is_empty());
    assert_eq!(environments.len(), 1);
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_route_list_environments_offset_success(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    // First request - get first item
    let first_response = actix_web::test::TestRequest::get()
        .uri("/environments?limit=1")
        .send_request(&app)
        .await;

    // Second request - get second item using offset
    let second_response = actix_web::test::TestRequest::get()
        .uri("/environments?offset=1&limit=1")
        .send_request(&app)
        .await;

    let first_env: Vec<EnvironmentResponse> = actix_web::test::read_body_json(first_response).await;
    let second_env: Vec<EnvironmentResponse> = actix_web::test::read_body_json(second_response).await;

    assert_eq!(first_env.len(), 1);
    assert_eq!(second_env.len(), 1);
    assert_ne!(
        first_env[0].id, second_env[0].id,
        "Offset pagination failed - returned same record"
    );
}
