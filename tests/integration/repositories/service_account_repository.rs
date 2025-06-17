use std::sync::Arc;

use sentinel_guard::{
    models::{
        pagination::Pagination,
        service_account::{
            ServiceAccountCreatePayload, ServiceAccountFilter, ServiceAccountSortOrder,
            ServiceAccountSortableFields, ServiceAccountUpdatePayload,
        },
        sort::SortOrder,
    },
    repositories::{base::Repository, service_account_repository::ServiceAccountRepository},
    utils::security::SecretsManager,
};
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test]
async fn test_service_account_repository_create_with_valid_data_succeeds(pool: PgPool) {
    let repository = ServiceAccountRepository::new(Arc::new(pool));

    let payload = ServiceAccountCreatePayload {
        name: "Test Service Account".to_string(),
        email: "test@example.com".to_string(),
        secret: "supersecret".to_string(),
        description: "Test Description".to_string(),
        enabled: true,
    };

    let service_account = repository.create(payload.clone()).await.unwrap();

    let secrets_manager = SecretsManager::new(true).unwrap();
    let decrypt_password = secrets_manager
        .decrypt(&service_account.secret, &service_account.id.unwrap())
        .unwrap();

    assert_eq!(service_account.name, "Test Service Account");
    assert_eq!(service_account.email, "test@example.com");
    assert_eq!(decrypt_password, "supersecret");
    assert_eq!(service_account.description, "Test Description");
    assert!(service_account.enabled);
}

#[sqlx::test]
async fn test_service_account_repository_create_with_duplicate_name_fails(pool: PgPool) {
    let repository = ServiceAccountRepository::new(Arc::new(pool));

    let mut payload = ServiceAccountCreatePayload {
        name: "Test Service Account".to_string(),
        email: "test1@example.com".to_string(),
        secret: "supersecret".to_string(),
        description: "Test Description".to_string(),
        enabled: true,
    };

    // First create should succeed
    let _ = repository.create(payload.clone()).await.unwrap();

    payload.email = "test2@example.com".to_string();

    // Second create with same email should fail
    let result = repository.create(payload).await;
    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();

    dbg!(&error_message);

    assert!(error_message.contains("Failed to create service account"));
    assert!(error_message.contains("idx_service_account_name"));
    assert!(error_message.contains("duplicate key"));
}

#[sqlx::test]
async fn test_service_account_repository_create_with_duplicate_email_fails(pool: PgPool) {
    let repository = ServiceAccountRepository::new(Arc::new(pool));

    let mut payload = ServiceAccountCreatePayload {
        name: "Test Service Account".to_string(),
        email: "test1@example.com".to_string(),
        secret: "supersecret".to_string(),
        description: "Test Description".to_string(),
        enabled: true,
    };

    // First create should succeed
    let _ = repository.create(payload.clone()).await.unwrap();

    payload.name = "different name".to_string();

    // Second create with same email should fail
    let result = repository.create(payload).await;
    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();

    dbg!(&error_message);

    assert!(error_message.contains("Failed to create service account"));
    assert!(error_message.contains("idx_service_account_email"));
    assert!(error_message.contains("duplicate key"));
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_repository_read_existing_account_succeeds(pool: PgPool) {
    let repository = ServiceAccountRepository::new(Arc::new(pool));

    let service_account = repository
        .read(Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap())
        .await
        .unwrap();

    assert!(service_account.is_some());
    let service_account = service_account.unwrap();
    assert_eq!(service_account.name, "Test Account 1");
    assert_eq!(service_account.email, "test1@example.com");
    assert_eq!(service_account.description, "Test Description 1");
    assert!(service_account.enabled);
}

#[sqlx::test]
async fn test_service_account_repository_read_nonexistent_account_returns_error(pool: PgPool) {
    let repository = ServiceAccountRepository::new(Arc::new(pool));

    let result = repository
        .read(Uuid::parse_str("123e4567-e89b-12d3-a456-426614174999").unwrap())
        .await;

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Service account not found");
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_repository_update_name_field_succeeds(pool: PgPool) {
    test_service_account_repository_update_helper(
        pool,
        ServiceAccountUpdatePayload {
            name: Some("Updated Name".to_string()),
            email: None,
            secret: None,
            description: None,
            enabled: None,
        },
        |account| assert_eq!(account.name, "Updated Name"),
    )
    .await;
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_repository_update_email_field_succeeds(pool: PgPool) {
    test_service_account_repository_update_helper(
        pool,
        ServiceAccountUpdatePayload {
            name: None,
            email: Some("updated@example.com".to_string()),
            secret: None,
            description: None,
            enabled: None,
        },
        |account| assert_eq!(account.email, "updated@example.com"),
    )
    .await;
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_repository_update_secret_field_succeeds(pool: PgPool) {
    test_service_account_repository_update_helper(
        pool,
        ServiceAccountUpdatePayload {
            name: None,
            email: None,
            secret: Some("new-secret".to_string()),
            description: None,
            enabled: None,
        },
        |account| {
            let secrets_manager = SecretsManager::new(true).unwrap();
            let decrypt_password = secrets_manager
                .decrypt(&account.secret, &account.id.unwrap())
                .unwrap();
            assert_eq!(decrypt_password, "new-secret");
        },
    )
    .await;
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_repository_update_description_field_succeeds(pool: PgPool) {
    test_service_account_repository_update_helper(
        pool,
        ServiceAccountUpdatePayload {
            name: None,
            email: None,
            secret: None,
            description: Some("Updated Description".to_string()),
            enabled: None,
        },
        |account| assert_eq!(account.description, "Updated Description"),
    )
    .await;
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_repository_update_duplicate_name_fails(pool: PgPool) {
    let repository = ServiceAccountRepository::new(Arc::new(pool));

    let payload = ServiceAccountUpdatePayload {
        name: Some("Test Account 2".to_string()),
        email: None,
        secret: None,
        description: None,
        enabled: None,
    };

    // Second update with same name should fail
    let result = repository
        .update(
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap(),
            payload,
        )
        .await;
    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();

    assert!(error_message.contains("Service account with this name already exists"));
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_repository_update_duplicate_email_fails(pool: PgPool) {
    let repository = ServiceAccountRepository::new(Arc::new(pool));

    let payload = ServiceAccountUpdatePayload {
        name: None,
        email: Some("test2@example.com".to_string()),
        secret: None,
        description: None,
        enabled: None,
    };

    // Second update with same name should fail
    let result = repository
        .update(
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap(),
            payload,
        )
        .await;
    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();

    assert!(error_message.contains("Service account with this email already exists"));
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_repository_update_enabled_to_false_succeeds(pool: PgPool) {
    test_service_account_repository_update_helper(
        pool,
        ServiceAccountUpdatePayload {
            name: None,
            email: None,
            secret: None,
            description: None,
            enabled: Some(false),
        },
        |account| assert!(!account.enabled),
    )
    .await;
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_repository_update_nonexistent_account_returns_error(pool: PgPool) {
    let repository = ServiceAccountRepository::new(Arc::new(pool));

    let result = repository
        .update(
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174999").unwrap(),
            ServiceAccountUpdatePayload {
                name: Some("Updated".to_string()),
                email: None,
                secret: None,
                description: None,
                enabled: None,
            },
        )
        .await;

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Service account not found");
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_repository_delete_existing_account_succeeds(pool: PgPool) {
    let repository = ServiceAccountRepository::new(Arc::new(pool));

    let is_deleted = repository
        .delete(Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap())
        .await
        .unwrap();

    assert!(is_deleted);
}

#[sqlx::test]
async fn test_service_account_repository_delete_nonexistent_account_returns_false(pool: PgPool) {
    let repository = ServiceAccountRepository::new(Arc::new(pool));

    let result = repository
        .delete(Uuid::parse_str("123e4567-e89b-12d3-a456-426614174999").unwrap())
        .await;

    assert!(result.is_ok());
    assert!(!result.unwrap());
}

async fn test_service_account_repository_update_helper<F>(
    pool: PgPool,
    payload: ServiceAccountUpdatePayload,
    assertion: F,
) where
    F: FnOnce(&sentinel_guard::models::service_account::ServiceAccount),
{
    let repository = ServiceAccountRepository::new(Arc::new(pool));

    let service_account = repository
        .update(
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap(),
            payload,
        )
        .await
        .unwrap();

    assertion(&service_account);
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_repository_find_with_no_filters_returns_all_accounts(pool: PgPool) {
    let repository = ServiceAccountRepository::new(Arc::new(pool));

    let filter = ServiceAccountFilter::default();
    let sort = None;
    let pagination = None;

    let accounts = repository.find(filter, sort, pagination).await.unwrap();

    assert_eq!(accounts.len(), 3);
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_repository_find_with_name_filter(pool: PgPool) {
    let repository = ServiceAccountRepository::new(Arc::new(pool));

    let filter = ServiceAccountFilter {
        name: Some("Test Account 1".to_string()),
        ..Default::default()
    };
    let sort = None;
    let pagination = None;

    let accounts = repository.find(filter, sort, pagination).await.unwrap();

    assert_eq!(accounts.len(), 1);
    assert_eq!(accounts[0].name, "Test Account 1");
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_repository_find_with_email_filter(pool: PgPool) {
    let repository = ServiceAccountRepository::new(Arc::new(pool));

    let filter = ServiceAccountFilter {
        email: Some("test2@example.com".to_string()),
        ..Default::default()
    };
    let sort = None;
    let pagination = None;

    let accounts = repository.find(filter, sort, pagination).await.unwrap();

    assert_eq!(accounts.len(), 1);
    assert_eq!(accounts[0].email, "test2@example.com");
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_repository_find_with_enabled_filter(pool: PgPool) {
    let repository = ServiceAccountRepository::new(Arc::new(pool));

    let filter = ServiceAccountFilter {
        enabled: Some(true),
        ..Default::default()
    };
    let sort = None;
    let pagination = None;

    let accounts = repository.find(filter, sort, pagination).await.unwrap();

    assert_eq!(accounts.len(), 2);
    assert!(accounts[0].enabled);
    assert!(accounts[1].enabled);
}

#[sqlx::test(fixtures("../fixtures/sort_service_accounts.sql"))]
async fn test_service_account_repository_find_with_name_ascending_sort(pool: PgPool) {
    let repository = ServiceAccountRepository::new(Arc::new(pool));

    let filter = ServiceAccountFilter::default();
    let sort = Some(vec![ServiceAccountSortOrder::new(
        ServiceAccountSortableFields::Name,
        SortOrder::Asc,
    )]);
    let pagination = None;

    let accounts = repository.find(filter, sort, pagination).await.unwrap();

    assert_eq!(accounts.len(), 3);
    assert_eq!(accounts[0].name, "Account A");
    assert_eq!(accounts[1].name, "Account B");
    assert_eq!(accounts[2].name, "Account C");
}

#[sqlx::test(fixtures("../fixtures/sort_service_accounts.sql"))]
async fn test_service_account_repository_find_with_name_descending_sort(pool: PgPool) {
    let repository = ServiceAccountRepository::new(Arc::new(pool));

    let filter = ServiceAccountFilter::default();
    let sort = Some(vec![ServiceAccountSortOrder::new(
        ServiceAccountSortableFields::Name,
        SortOrder::Desc,
    )]);
    let pagination = None;

    let accounts = repository.find(filter, sort, pagination).await.unwrap();

    assert_eq!(accounts.len(), 3);
    assert_eq!(accounts[0].name, "Account C");
    assert_eq!(accounts[1].name, "Account B");
    assert_eq!(accounts[2].name, "Account A");
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_repository_find_with_limit_pagination(pool: PgPool) {
    let repository = ServiceAccountRepository::new(Arc::new(pool));

    let filter = ServiceAccountFilter::default();
    let sort = None;
    let pagination = Some(Pagination {
        limit: Some(2),
        offset: None,
    });

    let accounts = repository.find(filter, sort, pagination).await.unwrap();

    assert_eq!(accounts.len(), 2);
}

#[sqlx::test(fixtures("../fixtures/service_accounts.sql"))]
async fn test_service_account_repository_find_with_offset_pagination(pool: PgPool) {
    let repository = ServiceAccountRepository::new(Arc::new(pool));

    let filter = ServiceAccountFilter::default();
    let sort = None;
    let pagination = Some(Pagination {
        limit: None,
        offset: Some(1),
    });

    let accounts = repository.find(filter, sort, pagination).await.unwrap();

    assert_eq!(accounts.len(), 2);
}
