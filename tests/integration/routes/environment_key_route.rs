use std::sync::Arc;

use sqlx::PgPool;

use sentinel_guard::{
    models::environment_key::{
        EnvironmentKeyCreatePayload, EnvironmentKeyResponse, EnvironmentKeyUpdatePayload,
    },
    repositories::environment_key_repository::EnvironmentKeyRepository,
    routes::environment_key_route,
};

fn repositories(pool: PgPool) -> EnvironmentKeyRepository {
    EnvironmentKeyRepository::new(Arc::new(pool))
}

fn routes() -> fn(&mut actix_web::web::ServiceConfig) {
    environment_key_route::configure_routes
}

use crate::create_test_app;

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_environment_key_route_create_valid(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let payload = EnvironmentKeyCreatePayload {
        environment_id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        algorithm: "RS512".to_string(),
        active: true,
    };
    let response = actix_web::test::TestRequest::post()
        .uri("/environment-keys")
        .set_json(&payload)
        .send_request(&app)
        .await;

    assert_eq!(response.status(), actix_web::http::StatusCode::CREATED);
    let created: EnvironmentKeyResponse = actix_web::test::read_body_json(response).await;
    assert_eq!(created.environment_id, payload.environment_id);
    assert_eq!(created.algorithm, payload.algorithm);
    assert!(created.active);
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_environment_key_route_create_duplicate_fails(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let payload = EnvironmentKeyCreatePayload {
        environment_id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        algorithm: "HS256".to_string(),
        active: true,
    };
    let response = actix_web::test::TestRequest::post()
        .uri("/environment-keys")
        .set_json(&payload)
        .send_request(&app)
        .await;
    assert_eq!(response.status(), actix_web::http::StatusCode::BAD_REQUEST);
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_environment_key_route_get_by_id_succeeds(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let response = actix_web::test::TestRequest::get()
        .uri("/environment-keys/00000000-0000-0000-0000-000000000001")
        .send_request(&app)
        .await;
    assert!(response.status().is_success());
    let key: Option<EnvironmentKeyResponse> = actix_web::test::read_body_json(response).await;
    assert_eq!(key.unwrap().id, "00000000-0000-0000-0000-000000000001");
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_environment_key_route_get_by_id_not_found(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let response = actix_web::test::TestRequest::get()
        .uri("/environment-keys/00000000-0000-0000-0000-00000000dead")
        .send_request(&app)
        .await;
    assert!(response.status().is_client_error());
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_environment_key_route_patch_active_succeeds(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let payload = EnvironmentKeyUpdatePayload {
        active: Some(false),
    };
    let response = actix_web::test::TestRequest::patch()
        .uri("/environment-keys/00000000-0000-0000-0000-000000000001")
        .set_json(&payload)
        .send_request(&app)
        .await;
    assert!(response.status().is_success());
    let updated: EnvironmentKeyResponse = actix_web::test::read_body_json(response).await;
    assert!(!updated.active);
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_environment_key_route_patch_not_found(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let payload = EnvironmentKeyUpdatePayload {
        active: Some(false),
    };
    let response = actix_web::test::TestRequest::patch()
        .uri("/environment-keys/00000000-0000-0000-0000-00000000dead")
        .set_json(&payload)
        .send_request(&app)
        .await;
    assert!(response.status().is_client_error());
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_environment_key_route_patch_empty_payload(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let payload = EnvironmentKeyUpdatePayload { active: None };
    let response = actix_web::test::TestRequest::patch()
        .uri("/environment-keys/00000000-0000-0000-0000-000000000001")
        .set_json(&payload)
        .send_request(&app)
        .await;
    assert!(response.status().is_client_error());
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_environment_key_route_delete_succeeds(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let response = actix_web::test::TestRequest::delete()
        .uri("/environment-keys/00000000-0000-0000-0000-000000000001")
        .send_request(&app)
        .await;
    assert!(response.status().is_success());
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_environment_key_route_delete_not_found(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let response = actix_web::test::TestRequest::delete()
        .uri("/environment-keys/00000000-0000-0000-0000-00000000dead")
        .send_request(&app)
        .await;
    assert!(response.status().is_client_error());
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_environment_key_route_list_all(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let response = actix_web::test::TestRequest::get()
        .uri("/environment-keys")
        .send_request(&app)
        .await;
    assert!(response.status().is_success());
    let keys: Vec<EnvironmentKeyResponse> = actix_web::test::read_body_json(response).await;
    assert!(!keys.is_empty());
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_environment_key_route_list_filter_by_active(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let response = actix_web::test::TestRequest::get()
        .uri("/environment-keys?active=false")
        .send_request(&app)
        .await;

    assert!(response.status().is_success());
    let keys: Vec<EnvironmentKeyResponse> = actix_web::test::read_body_json(response).await;
    assert!(!keys.is_empty());
    assert!(keys.iter().all(|k| !k.active));
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_environment_key_route_rotate_key_succeeds(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let key_id = "00000000-0000-0000-0000-000000000001";
    let response = actix_web::test::TestRequest::post()
        .uri(&format!("/environment-keys/{}/rotate", key_id))
        .send_request(&app)
        .await;
    assert!(response.status().is_success());

    let body: serde_json::Value = actix_web::test::read_body_json(response).await;
    assert_eq!(body["id"], key_id);
    assert_eq!(body["message"], "Environment key rotated successfully");
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_environment_key_route_rotate_key_not_found(pool: PgPool) {
    let app = create_test_app!(repositories(pool), routes());
    let key_id = "00000000-0000-0000-0000-00000000dead";
    let response = actix_web::test::TestRequest::post()
        .uri(&format!("/environment-keys/{}/rotate", key_id))
        .send_request(&app)
        .await;
    assert!(response.status().is_client_error());
}
