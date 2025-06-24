use std::sync::Arc;

use sentinel_guard::{
    models::{
        pagination::Pagination,
        project_access_scopes::{
            ProjectAccessScopeCreatePayload, ProjectAccessScopeFilter,
            ProjectAccessScopeUpdatePayload,
        },
    },
    repositories::{
        base::Repository, project_access_scopes_repository::ProjectAccessScopesRepository,
    },
};
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_repository_create_with_valid_data_succeeds(pool: PgPool) {
    let repository = ProjectAccessScopesRepository::new(Arc::new(pool));
    let payload = ProjectAccessScopeCreatePayload {
        project_access_id: "00000000-0000-0000-0000-000000000102".to_string(),
        scope_id: "00000000-0000-0000-0000-000000000002".to_string(),
    };
    let scope = repository.create(payload.clone()).await.unwrap();
    assert_eq!(
        scope.project_access_id,
        Uuid::parse_str("00000000-0000-0000-0000-000000000102").unwrap()
    );
    assert_eq!(
        scope.scope_id,
        Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap()
    );
    assert!(scope.enabled);
}

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_repository_create_with_missing_project_access_id_fails(
    pool: PgPool,
) {
    let repository = ProjectAccessScopesRepository::new(Arc::new(pool));
    let payload = ProjectAccessScopeCreatePayload {
        project_access_id: Uuid::new_v4().to_string(),
        scope_id: "00000000-0000-0000-0000-000000000001".to_string(),
    };
    let result = repository.create(payload).await;
    assert!(result.is_err());
}

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_repository_create_with_duplicate_fails(pool: PgPool) {
    let repository = ProjectAccessScopesRepository::new(Arc::new(pool));
    let payload = ProjectAccessScopeCreatePayload {
        project_access_id: "00000000-0000-0000-0000-000000000101".to_string(),
        scope_id: "00000000-0000-0000-0000-000000000001".to_string(),
    };
    let result = repository.create(payload).await;
    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();
    assert_eq!(
        error_message,
        "Project Access Id and Scope Id combination already exists"
    );
}

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_repository_read_existing_succeeds(pool: PgPool) {
    let repository = ProjectAccessScopesRepository::new(Arc::new(pool));
    let id = Uuid::parse_str("00000000-0000-0000-0000-000000001001").unwrap();
    let scope = repository.read(id).await.unwrap();
    assert!(scope.is_some());
    let scope = scope.unwrap();
    assert_eq!(scope.id, Some(id));
    assert_eq!(
        scope.project_access_id,
        Uuid::parse_str("00000000-0000-0000-0000-000000000101").unwrap()
    );
}

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_repository_read_nonexistent_returns_error(pool: PgPool) {
    let repository = ProjectAccessScopesRepository::new(Arc::new(pool));
    let id = Uuid::new_v4();
    let result = repository.read(id).await;
    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();
    assert_eq!(error_message, "Project access scope not found");
}

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_repository_update_enabled_false_succeeds(pool: PgPool) {
    let repository = ProjectAccessScopesRepository::new(Arc::new(pool));
    let id = Uuid::parse_str("00000000-0000-0000-0000-000000001001").unwrap();
    let update = ProjectAccessScopeUpdatePayload {
        enabled: Some(false),
    };
    let scope = repository.update(id, update).await.unwrap();
    assert!(!scope.enabled);
}

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_repository_update_enabled_true_succeeds(pool: PgPool) {
    let repository = ProjectAccessScopesRepository::new(Arc::new(pool));
    let id = Uuid::parse_str("00000000-0000-0000-0000-000000001002").unwrap();
    let update = ProjectAccessScopeUpdatePayload {
        enabled: Some(true),
    };
    let scope = repository.update(id, update).await.unwrap();
    assert!(scope.enabled);
}

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_delete_existing_succeeds(pool: PgPool) {
    let repository = ProjectAccessScopesRepository::new(Arc::new(pool));
    let id = Uuid::parse_str("00000000-0000-0000-0000-000000001001").unwrap();
    let deleted = repository.delete(id).await.unwrap();
    assert!(deleted);
}

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_delete_nonexisting_fails(pool: PgPool) {
    let repository = ProjectAccessScopesRepository::new(Arc::new(pool));
    let id = Uuid::new_v4();
    let result = repository.delete(id).await;
    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();
    assert_eq!(error_message, "Project access scope not found");
}

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_find_with_limit_pagination(pool: PgPool) {
    let repository = ProjectAccessScopesRepository::new(Arc::new(pool));
    let filter = ProjectAccessScopeFilter::default();
    let sort = None;
    let pagination = Some(Pagination {
        limit: Some(2),
        offset: None,
    });
    let scopes = repository.find(filter, sort, pagination).await.unwrap();
    assert_eq!(scopes.len(), 2);
}

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_find_with_offset_pagination(pool: PgPool) {
    let repository = ProjectAccessScopesRepository::new(Arc::new(pool));
    let filter = ProjectAccessScopeFilter::default();
    let sort = None;
    let pagination = Some(Pagination {
        limit: None,
        offset: Some(1),
    });
    let scopes = repository.find(filter, sort, pagination).await.unwrap();
    assert_eq!(scopes.len(), 2);
}

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_find_with_limit_offset_pagination(pool: PgPool) {
    let repository = ProjectAccessScopesRepository::new(Arc::new(pool));
    let filter = ProjectAccessScopeFilter::default();
    let sort = None;
    let pagination = Some(Pagination {
        limit: Some(1),
        offset: Some(1),
    });
    let scopes = repository.find(filter, sort, pagination).await.unwrap();
    assert_eq!(scopes.len(), 1);
}

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_find_with_project_access_id_filter(pool: PgPool) {
    let repository = ProjectAccessScopesRepository::new(Arc::new(pool));
    let filter = ProjectAccessScopeFilter {
        project_access_id: Some("00000000-0000-0000-0000-000000000101".to_string()),
        ..Default::default()
    };
    let sort = None;
    let pagination = None;
    let scopes = repository.find(filter, sort, pagination).await.unwrap();
    assert_eq!(scopes.len(), 2);
}

#[sqlx::test(fixtures("../fixtures/project_access_scopes.sql"))]
async fn test_project_access_scopes_find_with_scope_id_filter(pool: PgPool) {
    let repository = ProjectAccessScopesRepository::new(Arc::new(pool));
    let filter = ProjectAccessScopeFilter {
        scope_id: Some("00000000-0000-0000-0000-000000000003".to_string()),
        ..Default::default()
    };
    let sort = None;
    let pagination = None;
    let scopes = repository.find(filter, sort, pagination).await.unwrap();
    assert_eq!(scopes.len(), 1);
}
