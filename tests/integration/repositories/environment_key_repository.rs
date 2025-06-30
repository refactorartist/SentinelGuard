use std::sync::Arc;

use sentinel_guard::models::environment_key::{
    EnvironmentKeyCreatePayload, EnvironmentKeyFilter, EnvironmentKeySortOrder,
    EnvironmentKeySortableFields, EnvironmentKeyUpdatePayload,
};
use sentinel_guard::models::sort::SortOrder;
use sentinel_guard::repositories::base::Repository;
use sentinel_guard::repositories::environment_key_repository::EnvironmentKeyRepository;
use sqlx::PgPool;
use uuid::Uuid;

// CREATE
#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_create_environment_key_valid(pool: PgPool) {
    let repo = EnvironmentKeyRepository::new(Arc::new(pool));
    let payload = EnvironmentKeyCreatePayload {
        environment_id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        algorithm: "HS384".to_string(),
        active: true,
    };
    let created = repo.create(payload).await.unwrap();
    assert_eq!(
        created.environment_id,
        Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap()
    );
    assert_eq!(format!("{:?}", created.algorithm), "HS384");
    assert!(created.active);
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_create_environment_key_duplicate_fails(pool: PgPool) {
    let repo = EnvironmentKeyRepository::new(Arc::new(pool));
    let payload = EnvironmentKeyCreatePayload {
        environment_id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        algorithm: "HS256".to_string(), // already exists in fixture
        active: true,
    };
    let result = repo.create(payload).await;
    dbg!(&result);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Environment Id and Algorithm combination already exists"
    );
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_create_environment_key_missing_env_fails(pool: PgPool) {
    let repo = EnvironmentKeyRepository::new(Arc::new(pool));
    let payload = EnvironmentKeyCreatePayload {
        environment_id: Uuid::new_v4().to_string(), // non-existent
        algorithm: "HS512".to_string(),
        active: true,
    };
    let result = repo.create(payload).await;
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Foreign key constraint failed"
    );
}

// READ
#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_read_environment_key_existing(pool: PgPool) {
    let repo = EnvironmentKeyRepository::new(Arc::new(pool));
    let key = repo
        .read(Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap())
        .await
        .unwrap();
    assert!(key.is_some());
    let key = key.unwrap();
    assert_eq!(
        key.id.unwrap(),
        Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()
    );
    assert_eq!(format!("{:?}", key.algorithm), "HS256");
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_read_environment_key_nonexistent(pool: PgPool) {
    let repo = EnvironmentKeyRepository::new(Arc::new(pool));
    let result = repo.read(Uuid::new_v4()).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Environment key not found");
}

// UPDATE
#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_update_environment_key_active(pool: PgPool) {
    let repo = EnvironmentKeyRepository::new(Arc::new(pool));
    let update = EnvironmentKeyUpdatePayload {
        active: Some(false),
    };
    let updated = repo
        .update(
            Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
            update,
        )
        .await;
    assert!(updated.is_ok());
    let updated = updated.unwrap();
    assert_eq!(
        updated.id.unwrap(),
        Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap()
    );
    assert!(!updated.active);
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_update_environment_key_no_changes(pool: PgPool) {
    let repo = EnvironmentKeyRepository::new(Arc::new(pool));
    let update = EnvironmentKeyUpdatePayload { active: None };
    let result = repo
        .update(
            Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
            update,
        )
        .await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "No changes to update");
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_update_environment_key_nonexistent(pool: PgPool) {
    let repo = EnvironmentKeyRepository::new(Arc::new(pool));
    let update = EnvironmentKeyUpdatePayload { active: Some(true) };
    let result = repo.update(Uuid::new_v4(), update).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Environment key not found");
}

// DELETE
#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_delete_environment_key_existing(pool: PgPool) {
    let repo = EnvironmentKeyRepository::new(Arc::new(pool));
    let deleted = repo
        .delete(Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap())
        .await
        .unwrap();
    assert!(deleted);
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_delete_environment_key_nonexistent(pool: PgPool) {
    let repo = EnvironmentKeyRepository::new(Arc::new(pool));
    let result = repo.delete(Uuid::new_v4()).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Environment key not found");
}

// FIND
#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_find_environment_keys_by_env(pool: PgPool) {
    let repo = EnvironmentKeyRepository::new(Arc::new(pool));
    let filter = EnvironmentKeyFilter {
        environment_id: Some("123e4567-e89b-12d3-a456-426614174000".to_string()),
        algorithm: None,
        active: None,
    };
    let sort = Some(vec![EnvironmentKeySortOrder::new(
        EnvironmentKeySortableFields::Algorithm,
        SortOrder::Asc,
    )]);
    let keys = repo.find(filter, sort, None).await.unwrap();
    assert!(!keys.is_empty());
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_find_environment_keys_with_pagination(pool: PgPool) {
    use sentinel_guard::models::pagination::Pagination;
    let repo = EnvironmentKeyRepository::new(Arc::new(pool));
    let filter = EnvironmentKeyFilter::default();
    let sort = None;
    let pagination = Some(Pagination {
        limit: Some(1),
        offset: Some(0),
    });
    let keys = repo.find(filter, sort, pagination).await.unwrap();
    assert_eq!(keys.len(), 1);
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_get_environment_key_rsa(pool: PgPool) {
    let repo = EnvironmentKeyRepository::new(Arc::new(pool));
    let environment_id = uuid::Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap();
    let algorithm = jsonwebtoken::Algorithm::RS512;

    let _ = repo
        .create(EnvironmentKeyCreatePayload {
            environment_id: environment_id.to_string(),
            algorithm: format!("{:?}", algorithm),
            active: true,
        })
        .await
        .unwrap();

    let key = repo
        .get_environment_key(environment_id, algorithm)
        .await
        .unwrap();
    assert!(key.contains("-----BEGIN PRIVATE KEY-----"));
    assert!(key.contains("-----END PRIVATE KEY-----"));
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_get_environment_key_hmac(pool: PgPool) {
    let repo = EnvironmentKeyRepository::new(Arc::new(pool));
    let environment_id = uuid::Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap();
    let algorithm = jsonwebtoken::Algorithm::HS384;

    let _ = repo
        .create(EnvironmentKeyCreatePayload {
            environment_id: environment_id.to_string(),
            algorithm: format!("{:?}", algorithm),
            active: true,
        })
        .await
        .unwrap();

    let key: String = repo
        .get_environment_key(environment_id, algorithm)
        .await
        .unwrap();
    assert!(!key.is_empty());
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_rotate_key_existing_active(pool: PgPool) {
    let repo = EnvironmentKeyRepository::new(Arc::new(pool));
    let id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();

    // Get the original key and updated_at
    let row = sqlx::query!(
        "SELECT key, updated_at FROM environment_key WHERE id = $1",
        id
    )
    .fetch_one(&*repo.pool)
    .await
    .unwrap();
    let original_key = row.key.clone();
    let original_updated_at = row.updated_at;

    // Rotate the key
    let rotated = repo.clone().rotate_key(id).await.unwrap();
    assert_eq!(rotated.id, Some(id));
    assert_eq!(
        rotated.environment_id,
        Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap()
    );
    assert!(rotated.updated_at > original_updated_at);

    // The key in the DB should be different
    let row = sqlx::query!(
        "SELECT key, updated_at FROM environment_key WHERE id = $1",
        id
    )
    .fetch_one(&*repo.pool)
    .await
    .unwrap();
    assert_ne!(row.key, original_key);
    assert!(row.updated_at > original_updated_at);
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_rotate_key_nonexistent(pool: PgPool) {
    let repo = EnvironmentKeyRepository::new(Arc::new(pool));
    let id = Uuid::new_v4();
    let result = repo.clone().rotate_key(id).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Environment key not found");
}

#[sqlx::test(fixtures("../fixtures/environment_keys.sql"))]
async fn test_rotate_key_inactive(pool: PgPool) {
    let repo = EnvironmentKeyRepository::new(Arc::new(pool));
    let id = Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap(); // inactive key

    // Get the original key and updated_at
    let row = sqlx::query!(
        "SELECT key, updated_at FROM environment_key WHERE id = $1",
        id
    )
    .fetch_one(&*repo.pool)
    .await
    .unwrap();
    let original_key = row.key.clone();
    let original_updated_at = row.updated_at;

    // Rotate the key (should still succeed)
    let rotated = repo.clone().rotate_key(id).await.unwrap();
    assert_eq!(rotated.id, Some(id));
    assert_eq!(
        rotated.environment_id,
        Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap()
    );
    assert!(rotated.updated_at > original_updated_at);

    // The key in the DB should be different
    let row = sqlx::query!(
        "SELECT key, updated_at FROM environment_key WHERE id = $1",
        id
    )
    .fetch_one(&*repo.pool)
    .await
    .unwrap();
    assert_ne!(row.key, original_key);
    assert!(row.updated_at > original_updated_at);
}
