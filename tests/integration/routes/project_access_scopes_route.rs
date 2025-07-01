use std::sync::Arc;

use sqlx::PgPool;

use sentinel_guard::{
    models::project_access_scopes::{
        ProjectAccessScopeCreatePayload, ProjectAccessScopeResponse,
        ProjectAccessScopeUpdatePayload,
    },
    repositories::project_access_scopes_repository::ProjectAccessScopesRepository,
    routes::project_access_scopes_route,
};

fn repositories(pool: PgPool) -> ProjectAccessScopesRepository {
    ProjectAccessScopesRepository::new(Arc::new(pool))
}

fn routes() -> fn(&mut actix_web::web::ServiceConfig) {
    project_access_scopes_route::configure_routes
}

use crate::create_test_app;

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_route_create_valid(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let payload = ProjectAccessScopeCreatePayload {
        project_access_id: "00000000-0000-0000-0000-000000000102".to_string(),
        scope_id: "00000000-0000-0000-0000-000000000002".to_string(),
    };
    let response = actix_web::test::TestRequest::post()
        .uri("/project-access-scopes")
        .set_json(&payload)
        .send_request(&app)
        .await;
    assert_eq!(response.status(), actix_web::http::StatusCode::CREATED);
    let created: ProjectAccessScopeResponse = actix_web::test::read_body_json(response).await;
    assert_eq!(created.project_access_id, payload.project_access_id);
    assert_eq!(created.scope_id, payload.scope_id);
    assert!(created.enabled);
}

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_route_create_duplicate_fails(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let payload = ProjectAccessScopeCreatePayload {
        project_access_id: "00000000-0000-0000-0000-000000000101".to_string(),
        scope_id: "00000000-0000-0000-0000-000000000001".to_string(),
    };
    let response = actix_web::test::TestRequest::post()
        .uri("/project-access-scopes")
        .set_json(&payload)
        .send_request(&app)
        .await;
    assert_eq!(response.status(), actix_web::http::StatusCode::BAD_REQUEST);
}

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_route_get_by_id_succeeds(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let response = actix_web::test::TestRequest::get()
        .uri("/project-access-scopes/00000000-0000-0000-0000-000000001001")
        .send_request(&app)
        .await;
    assert!(response.status().is_success());
    let scope: ProjectAccessScopeResponse = actix_web::test::read_body_json(response).await;
    assert_eq!(scope.id, "00000000-0000-0000-0000-000000001001");
}

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_route_get_by_id_not_found(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let response = actix_web::test::TestRequest::get()
        .uri("/project-access-scopes/00000000-0000-0000-0000-00000000dead")
        .send_request(&app)
        .await;
    assert!(response.status().is_client_error());
}

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_route_patch_enabled_succeeds(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let payload = ProjectAccessScopeUpdatePayload {
        enabled: Some(false),
    };
    let response = actix_web::test::TestRequest::patch()
        .uri("/project-access-scopes/00000000-0000-0000-0000-000000001001")
        .set_json(&payload)
        .send_request(&app)
        .await;
    assert!(response.status().is_success());
    let updated: ProjectAccessScopeResponse = actix_web::test::read_body_json(response).await;
    assert!(!updated.enabled);
}

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_route_patch_not_found(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let payload = ProjectAccessScopeUpdatePayload {
        enabled: Some(false),
    };
    let response = actix_web::test::TestRequest::patch()
        .uri("/project-access-scopes/00000000-0000-0000-0000-00000000dead")
        .set_json(&payload)
        .send_request(&app)
        .await;
    assert!(response.status().is_client_error());
}

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_route_patch_empty_payload(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let payload = ProjectAccessScopeUpdatePayload { enabled: None };
    let response = actix_web::test::TestRequest::patch()
        .uri("/project-access-scopes/00000000-0000-0000-0000-000000001001")
        .set_json(&payload)
        .send_request(&app)
        .await;
    assert!(response.status().is_client_error());
}

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_route_delete_succeeds(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let response = actix_web::test::TestRequest::delete()
        .uri("/project-access-scopes/00000000-0000-0000-0000-000000001001")
        .send_request(&app)
        .await;
    assert!(response.status().is_success());
}

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_route_delete_not_found(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let response = actix_web::test::TestRequest::delete()
        .uri("/project-access-scopes/00000000-0000-0000-0000-00000000dead")
        .send_request(&app)
        .await;
    assert!(response.status().is_client_error());
}

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_route_list_all(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let response = actix_web::test::TestRequest::get()
        .uri("/project-access-scopes")
        .send_request(&app)
        .await;
    assert!(response.status().is_success());
    let scopes: Vec<ProjectAccessScopeResponse> = actix_web::test::read_body_json(response).await;
    assert!(!scopes.is_empty());
}

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_route_list_filter_by_project_access_id(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let response = actix_web::test::TestRequest::get()
        .uri("/project-access-scopes?project_access_id=00000000-0000-0000-0000-000000000101")
        .send_request(&app)
        .await;
    assert!(response.status().is_success());
    let scopes: Vec<ProjectAccessScopeResponse> = actix_web::test::read_body_json(response).await;
    assert!(!scopes.is_empty());
    assert!(
        scopes
            .iter()
            .all(|s| s.project_access_id == "00000000-0000-0000-0000-000000000101")
    );
}
