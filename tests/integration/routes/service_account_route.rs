use std::sync::Arc;

use sqlx::PgPool;

use sentinel_guard::{
    models::service_account::{
        ServiceAccount, ServiceAccountCreatePayload, ServiceAccountUpdatePayload,
    },
    repositories::service_account_repository::ServiceAccountRepository,
    routes::service_account_route,
    services::service_account_service::ServiceAccountService,
};

use crate::create_test_app;

fn services(pool: PgPool) -> Vec<ServiceAccountService> {
    vec![ServiceAccountService::new(ServiceAccountRepository::new(
        Arc::new(pool),
    ))]
}

fn routes() -> Vec<fn(&mut actix_web::web::ServiceConfig)> {
    vec![service_account_route::configure_routes]
}

#[sqlx::test]
async fn test_service_account_route_create_service_account_with_valid_data_succeeds(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let service_account = ServiceAccountCreatePayload {
        name: "test".to_string(),
        description: "test".to_string(),
        enabled: true,
        email: "test@example.com".to_string(),
        secret: "test".to_string(),
    };

    let response = actix_web::test::TestRequest::post()
        .uri("/service-accounts")
        .set_json(&service_account)
        .send_request(&app)
        .await;

    assert!(response.status().is_success());

    let created_service_account: ServiceAccount = actix_web::test::read_body_json(response).await;

    assert_eq!(created_service_account.name, "test");
    assert_eq!(created_service_account.description, "test");
    assert!(created_service_account.enabled);
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_route_create_service_account_with_duplicate_name_fails(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let service_account = ServiceAccountCreatePayload {
        name: "Test Account 1".to_string(),
        description: "test".to_string(),
        enabled: true,
        email: "test@example.com".to_string(),
        secret: "something".to_string(),
    };

    let response = actix_web::test::TestRequest::post()
        .uri("/service-accounts")
        .set_json(&service_account)
        .send_request(&app)
        .await;

    assert!(response.status().is_client_error());

    assert_eq!(response.status(), actix_web::http::StatusCode::BAD_REQUEST);
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_route_create_service_account_with_duplicate_email_fails(
    pool: PgPool,
) {
    let app = create_test_app!(services(pool), routes());

    let service_account = ServiceAccountCreatePayload {
        name: "Test Account 1".to_string(),
        description: "test".to_string(),
        enabled: true,
        email: "test@example.com".to_string(),
        secret: "something".to_string(),
    };

    let response = actix_web::test::TestRequest::post()
        .uri("/service-accounts")
        .set_json(&service_account)
        .send_request(&app)
        .await;

    assert!(response.status().is_client_error());

    assert_eq!(response.status(), actix_web::http::StatusCode::BAD_REQUEST);
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_route_get_service_account_by_id_succeeds(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let response = actix_web::test::TestRequest::get()
        .uri("/service-accounts/123e4567-e89b-12d3-a456-426614174000")
        .send_request(&app)
        .await;

    assert!(response.status().is_success());

    let service_account: ServiceAccount = actix_web::test::read_body_json(response).await;

    assert_eq!(service_account.name, "Test Account 1");
    assert_eq!(service_account.description, "Test Description 1");
    assert_eq!(service_account.email, "test1@example.com");
    assert_eq!(service_account.secret, "secret1");
    assert!(service_account.enabled);
}

#[sqlx::test]
async fn test_service_account_route_get_service_account_by_id_not_found(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let response = actix_web::test::TestRequest::get()
        .uri("/service-accounts/123e4567-e89b-12d3-a456-426614174000")
        .send_request(&app)
        .await;

    assert!(response.status().is_client_error());

    assert_eq!(response.status(), actix_web::http::StatusCode::NOT_FOUND);
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_route_patch_service_account_name_succeeds(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let payload = ServiceAccountUpdatePayload {
        name: Some("testc".to_string()),
        description: None,
        enabled: None,
        email: None,
        secret: None,
    };

    let response = actix_web::test::TestRequest::patch()
        .uri("/service-accounts/123e4567-e89b-12d3-a456-426614174000")
        .set_json(&payload)
        .send_request(&app)
        .await;

    assert!(response.status().is_success());
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_route_patch_service_account_description_succeeds(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let payload = ServiceAccountUpdatePayload {
        name: None,
        description: Some("something to change immediately".to_string()),
        enabled: None,
        email: None,
        secret: None,
    };

    let response = actix_web::test::TestRequest::patch()
        .uri("/service-accounts/123e4567-e89b-12d3-a456-426614174001")
        .set_json(&payload)
        .send_request(&app)
        .await;

    let (_request, response) = response.into_parts();

    assert!(response.status().is_success());
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_route_patch_service_account_enabled_succeeds(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let payload = ServiceAccountUpdatePayload {
        name: None,
        description: None,
        enabled: Some(false),
        email: None,
        secret: None,
    };

    let response = actix_web::test::TestRequest::patch()
        .uri("/service-accounts/123e4567-e89b-12d3-a456-426614174000")
        .set_json(&payload)
        .send_request(&app)
        .await;

    assert!(response.status().is_success());
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_route_patch_service_account_empty_payload(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let payload = ServiceAccountUpdatePayload {
        name: None,
        description: None,
        enabled: None,
        email: None,
        secret: None,
    };

    let response = actix_web::test::TestRequest::patch()
        .uri("/service-accounts/123e4567-e89b-12d3-a456-426614174000")
        .set_json(&payload)
        .send_request(&app)
        .await;

    assert!(response.status().is_client_error());
    assert_eq!(response.status(), actix_web::http::StatusCode::BAD_REQUEST);
}

#[sqlx::test]
async fn test_service_account_route_patch_service_account_not_found(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let payload = ServiceAccountUpdatePayload {
        name: Some("test".to_string()),
        description: None,
        enabled: None,
        email: None,
        secret: None,
    };

    let response = actix_web::test::TestRequest::patch()
        .uri("/service-accounts/123e4567-e89b-12d3-a456-426614178000")
        .set_json(&payload)
        .send_request(&app)
        .await;

    assert!(response.status().is_client_error());
    assert_eq!(response.status(), actix_web::http::StatusCode::NOT_FOUND);
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_route_patch_service_account_duplicate_name_error(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let payload = ServiceAccountUpdatePayload {
        name: Some("Test Account 3".to_string()),
        description: None,
        enabled: None,
        email: None,
        secret: None,
    };

    let response = actix_web::test::TestRequest::patch()
        .uri("/service-accounts/123e4567-e89b-12d3-a456-426614174000")
        .set_json(&payload)
        .send_request(&app)
        .await;

    assert!(response.status().is_client_error());
    assert_eq!(response.status(), actix_web::http::StatusCode::CONFLICT);
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_route_patch_service_account_duplicate_email_error(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let payload = ServiceAccountUpdatePayload {
        name: None,
        description: None,
        enabled: None,
        email: Some("test3@example.com".to_string()),
        secret: None,
    };

    let response = actix_web::test::TestRequest::patch()
        .uri("/service-accounts/123e4567-e89b-12d3-a456-426614174000")
        .set_json(&payload)
        .send_request(&app)
        .await;

    assert!(response.status().is_client_error());
    assert_eq!(response.status(), actix_web::http::StatusCode::CONFLICT);
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_route_delete_service_account_succeeds(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let response = actix_web::test::TestRequest::delete()
        .uri("/service-accounts/123e4567-e89b-12d3-a456-426614174000")
        .send_request(&app)
        .await;

    assert!(response.status().is_success());
    assert_eq!(response.status(), actix_web::http::StatusCode::NO_CONTENT);
}

#[sqlx::test]
async fn test_service_account_route_delete_service_account_not_found(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());

    let response = actix_web::test::TestRequest::delete()
        .uri("/service-accounts/123e4567-e89b-12d3-a456-426614174000")
        .send_request(&app)
        .await;

    assert!(response.status().is_client_error());
    assert_eq!(response.status(), actix_web::http::StatusCode::NOT_FOUND);
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_route_list_service_accounts_returns_all(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());
    let response = actix_web::test::TestRequest::get()
        .uri("/service-accounts")
        .send_request(&app)
        .await;
    assert!(response.status().is_success());
    let service_accounts: Vec<ServiceAccount> = actix_web::test::read_body_json(response).await;
    // There are 3 service_accounts in the fixture
    assert_eq!(service_accounts.len(), 3);
    // Check some fields for correctness
    assert!(service_accounts.iter().any(|p| p.name == "Test Account 1"));
    assert!(service_accounts.iter().any(|p| p.name == "Test Account 2"));
    assert!(service_accounts.iter().any(|p| p.name == "Test Account 3"));
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_route_list_service_accounts_with_pagination(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());
    // limit=2, offset=1
    let response = actix_web::test::TestRequest::get()
        .uri("/service-accounts?limit=2&offset=1")
        .send_request(&app)
        .await;
    assert!(response.status().is_success());
    let service_accounts: Vec<ServiceAccount> = actix_web::test::read_body_json(response).await;
    assert_eq!(service_accounts.len(), 2);
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_route_list_service_accounts_filter_by_name(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());
    let response = actix_web::test::TestRequest::get()
        .uri("/service-accounts?name=Test")
        .send_request(&app)
        .await;
    assert!(response.status().is_success());
    let service_accounts: Vec<ServiceAccount> = actix_web::test::read_body_json(response).await;
    assert_eq!(service_accounts.len(), 3);
    assert!(service_accounts.iter().all(|p| p.name.contains("Test")));
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_route_list_service_accounts_filter_by_enabled_true(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());
    let response = actix_web::test::TestRequest::get()
        .uri("/service-accounts?enabled=true")
        .send_request(&app)
        .await;
    assert!(response.status().is_success());
    let service_accounts: Vec<ServiceAccount> = actix_web::test::read_body_json(response).await;
    assert!(!service_accounts.is_empty());
    assert!(service_accounts.iter().all(|p| p.enabled));
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_route_list_service_accounts_filter_by_enabled_false(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());
    let response = actix_web::test::TestRequest::get()
        .uri("/service-accounts?enabled=false")
        .send_request(&app)
        .await;
    assert!(response.status().is_success());
    let service_accounts: Vec<ServiceAccount> = actix_web::test::read_body_json(response).await;
    assert!(!service_accounts.is_empty());
    assert!(service_accounts.iter().all(|p| !p.enabled));
}

#[sqlx::test]
async fn test_service_account_route_list_service_accounts_empty(pool: PgPool) {
    let app = create_test_app!(services(pool), routes());
    let response = actix_web::test::TestRequest::get()
        .uri("/service-accounts")
        .send_request(&app)
        .await;
    assert!(response.status().is_success());
    let service_accounts: Vec<ServiceAccount> = actix_web::test::read_body_json(response).await;
    assert_eq!(service_accounts.len(), 0);
}
