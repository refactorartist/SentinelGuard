use std::sync::Arc;

use sentinel_guard::{models::project_scope::{ProjectScopeCreatePayload, ProjectScopeUpdatePayload}, repositories::project_scope_repository::ProjectScopeRepository, routes::project_scope_route, services::project_scope_service::ProjectScopeService};
use sqlx::PgPool;

use crate::{create_test_app};

fn services(pool: PgPool) -> ProjectScopeService {
    ProjectScopeService::new(ProjectScopeRepository::new(
        Arc::new(pool),
    ))
}

fn routes() -> fn(&mut actix_web::web::ServiceConfig) {
    project_scope_route::configure_routes
}


#[sqlx::test(fixtures("../fixtures/projects.sql"))]
async fn test_project_scope_route_create_project_scope_with_valid_data_succeeds(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let project_scope = ProjectScopeCreatePayload {
        scope: "test:changes-made".to_string(),
        description: "test".to_string(),
        enabled: true,
        project_id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
    };

    let response = actix_web::test::TestRequest::post()
        .uri("/project-scopes")
        .set_json(&project_scope)
        .send_request(&app)
        .await;

    assert!(response.status().is_success());
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/project_scopes.sql"))]
async fn test_project_scope_route_create_project_scope_with_duplicate_project_id_scope_fails(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let project_scope = ProjectScopeCreatePayload {
        scope: "testa:admin".to_string(),
        description: "test".to_string(),
        enabled: true,
        project_id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
    };

    let response = actix_web::test::TestRequest::post()
        .uri("/project-scopes")
        .set_json(&project_scope)
        .send_request(&app)
        .await;

    assert!(response.status().is_client_error());
    assert_eq!(response.status(), actix_web::http::StatusCode::BAD_REQUEST);
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/project_scopes.sql"))]
async fn test_project_scope_route_read_project_scope_by_id_successful(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let response = actix_web::test::TestRequest::get()
        .uri("/project-scopes/00000000-0000-0000-0000-000000000001")
        .send_request(&app)
        .await;

    assert!(response.status().is_success());
}


#[sqlx::test]
async fn test_project_scope_route_read_project_scope_by_id_not_found(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let response = actix_web::test::TestRequest::get()
        .uri("/project-scopes/00000000-0000-0000-0000-000000000002")
        .send_request(&app)
        .await;

    assert!(response.status().is_client_error());
    assert_eq!(response.status(), actix_web::http::StatusCode::NOT_FOUND);
}


#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/project_scopes.sql"))]
async fn test_project_scope_route_patch_scope_successful(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let payload = ProjectScopeUpdatePayload {
        scope: Some("test:changes-made".to_string()),
        description: None,
        enabled: None,
    };

    let response = actix_web::test::TestRequest::patch()
        .uri("/project-scopes/00000000-0000-0000-0000-000000000001")
        .set_json(&payload)
        .send_request(&app)
        .await;

    assert!(response.status().is_success());
}



#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/project_scopes.sql"))]
async fn test_project_scope_route_patch_description_successful(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let payload = ProjectScopeUpdatePayload {
        scope: None,
        description: Some("test:changes-made".to_string()),
        enabled: None,
    };

    let response = actix_web::test::TestRequest::patch()
        .uri("/project-scopes/00000000-0000-0000-0000-000000000001")
        .set_json(&payload)
        .send_request(&app)
        .await;

    assert!(response.status().is_success());
}


#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/project_scopes.sql"))]
async fn test_project_scope_route_patch_enabled_true_successful(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let payload = ProjectScopeUpdatePayload {
        scope: None,
        description: None,
        enabled: Some(true),
    };

    let response = actix_web::test::TestRequest::patch()
        .uri("/project-scopes/00000000-0000-0000-0000-000000000011")
        .set_json(&payload)
        .send_request(&app)
        .await;

    assert!(response.status().is_success());
}


#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/project_scopes.sql"))]
async fn test_project_scope_route_patch_enabled_false_successful(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let payload = ProjectScopeUpdatePayload {
        scope: None,
        description: None,
        enabled: Some(false),
    };

    let response = actix_web::test::TestRequest::patch()
        .uri("/project-scopes/00000000-0000-0000-0000-000000000001")
        .set_json(&payload)
        .send_request(&app)
        .await;

    assert!(response.status().is_success());
}
