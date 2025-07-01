use std::sync::Arc;

use sqlx::PgPool;

use sentinel_guard::{
    models::project_access::{
        ProjectAccessCreatePayload, ProjectAccessResponse, ProjectAccessUpdatePayload,
    },
    repositories::project_access_repository::ProjectAccessRepository,
    routes::project_access_route,
};

fn repositories(pool: PgPool) -> ProjectAccessRepository {
    ProjectAccessRepository::new(Arc::new(pool))
}

fn routes() -> fn(&mut actix_web::web::ServiceConfig) {
    project_access_route::configure_routes
}

use crate::create_test_app;

#[sqlx::test(fixtures("../fixtures/project_access.sql"))]
async fn test_project_access_route_create_valid(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let payload = ProjectAccessCreatePayload {
        project_id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        service_account_id: "123e4567-e89b-12d3-a456-426614174001".to_string(),
        environment_id: "00000000-0000-0000-0000-000000000001".to_string(),
        enabled: true,
    };
    let response = actix_web::test::TestRequest::post()
        .uri("/project-access")
        .set_json(&payload)
        .send_request(&app)
        .await;
    assert_eq!(response.status(), actix_web::http::StatusCode::CREATED);
    let created: ProjectAccessResponse = actix_web::test::read_body_json(response).await;
    assert_eq!(created.project_id, payload.project_id);
    assert_eq!(created.service_account_id, payload.service_account_id);
    assert_eq!(created.environment_id, payload.environment_id);
    assert!(created.enabled);
}

#[sqlx::test(fixtures("../fixtures/project_access.sql"))]
async fn test_project_access_route_create_duplicate_fails(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let payload = ProjectAccessCreatePayload {
        project_id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        service_account_id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        environment_id: "00000000-0000-0000-0000-000000000001".to_string(),
        enabled: true,
    };
    let response = actix_web::test::TestRequest::post()
        .uri("/project-access")
        .set_json(&payload)
        .send_request(&app)
        .await;

    println!("response: {:?}", response);
    assert_eq!(response.status(), actix_web::http::StatusCode::BAD_REQUEST);
}

#[sqlx::test(fixtures("../fixtures/project_access.sql"))]
async fn test_project_access_route_get_by_id_succeeds(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let response = actix_web::test::TestRequest::get()
        .uri("/project-access/00000000-0000-0000-0000-000000000101")
        .send_request(&app)
        .await;
    assert!(response.status().is_success());
    let access: ProjectAccessResponse = actix_web::test::read_body_json(response).await;
    assert_eq!(access.id, "00000000-0000-0000-0000-000000000101");
}

#[sqlx::test(fixtures("../fixtures/project_access.sql"))]
async fn test_project_access_route_get_by_id_not_found(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let response = actix_web::test::TestRequest::get()
        .uri("/project-access/00000000-0000-0000-0000-00000000dead")
        .send_request(&app)
        .await;
    assert!(response.status().is_client_error());
}

#[sqlx::test(fixtures("../fixtures/project_access.sql"))]
async fn test_project_access_route_patch_enabled_succeeds(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let payload = ProjectAccessUpdatePayload {
        enabled: Some(false),
    };
    let response = actix_web::test::TestRequest::patch()
        .uri("/project-access/00000000-0000-0000-0000-000000000101")
        .set_json(&payload)
        .send_request(&app)
        .await;
    assert!(response.status().is_success());
    let updated: ProjectAccessResponse = actix_web::test::read_body_json(response).await;
    assert!(!updated.enabled);
}

#[sqlx::test(fixtures("../fixtures/project_access.sql"))]
async fn test_project_access_route_patch_not_found(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let payload = ProjectAccessUpdatePayload {
        enabled: Some(false),
    };
    let response = actix_web::test::TestRequest::patch()
        .uri("/project-access/00000000-0000-0000-0000-00000000dead")
        .set_json(&payload)
        .send_request(&app)
        .await;
    assert!(response.status().is_client_error());
}

#[sqlx::test(fixtures("../fixtures/project_access.sql"))]
async fn test_project_access_route_patch_empty_payload(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let payload = ProjectAccessUpdatePayload { enabled: None };
    let response = actix_web::test::TestRequest::patch()
        .uri("/project-access/00000000-0000-0000-0000-000000000101")
        .set_json(&payload)
        .send_request(&app)
        .await;
    assert!(response.status().is_client_error());
}

#[sqlx::test(fixtures("../fixtures/project_access.sql"))]
async fn test_project_access_route_delete_succeeds(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let response = actix_web::test::TestRequest::delete()
        .uri("/project-access/00000000-0000-0000-0000-000000000101")
        .send_request(&app)
        .await;
    assert!(response.status().is_success());
}

#[sqlx::test(fixtures("../fixtures/project_access.sql"))]
async fn test_project_access_route_delete_not_found(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let response = actix_web::test::TestRequest::delete()
        .uri("/project-access/00000000-0000-0000-0000-00000000dead")
        .send_request(&app)
        .await;
    assert!(response.status().is_client_error());
}

#[sqlx::test(fixtures("../fixtures/project_access.sql"))]
async fn test_project_access_route_list_all(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let response = actix_web::test::TestRequest::get()
        .uri("/project-access")
        .send_request(&app)
        .await;
    assert!(response.status().is_success());
    let accesses: Vec<ProjectAccessResponse> = actix_web::test::read_body_json(response).await;
    assert!(!accesses.is_empty());
}

#[sqlx::test(fixtures("../fixtures/project_access.sql"))]
async fn test_project_access_route_list_filter_by_enabled(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let response = actix_web::test::TestRequest::get()
        .uri("/project-access?enabled=false")
        .send_request(&app)
        .await;
    assert!(response.status().is_success());
    let accesses: Vec<ProjectAccessResponse> = actix_web::test::read_body_json(response).await;
    assert!(!accesses.is_empty());
    assert!(accesses.iter().all(|a| !a.enabled));
}
