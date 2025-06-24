use std::sync::Arc;

use sentinel_guard::{
    models::{
        pagination::Pagination,
        project_access::{
            ProjectAccessCreatePayload, ProjectAccessFilter, ProjectAccessUpdatePayload,
        },
    },
    repositories::{base::Repository, project_access_repository::ProjectAccessRepository},
};
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test(fixtures(
    "../fixtures/projects.sql",
    "../fixtures/service_accounts.sql",
    "../fixtures/environments.sql"
))]
async fn test_project_access_repository_create_with_valid_data_succeeds(pool: PgPool) {
    let repository = ProjectAccessRepository::new(Arc::new(pool));
    let payload = ProjectAccessCreatePayload {
        project_id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        service_account_id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        environment_id: "00000000-0000-0000-0000-000000000001".to_string(),
        enabled: true,
    };
    let project_access = repository.create(payload.clone()).await.unwrap();
    assert_eq!(project_access.project_id, Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap());
    assert_eq!(project_access.service_account_id, Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap());
    assert_eq!(project_access.environment_id, Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap());
    assert!(project_access.enabled);
}

#[sqlx::test]
async fn test_project_access_repository_create_with_missing_project_id_fails(pool: PgPool) {
    let repository = ProjectAccessRepository::new(Arc::new(pool));
    let payload = ProjectAccessCreatePayload {
        project_id: Uuid::new_v4().to_string(),
        service_account_id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        environment_id: "00000000-0000-0000-0000-000000000001".to_string(),
        enabled: true,
    };
    let result = repository.create(payload).await;
    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();
    assert_eq!(error_message, "Project not found");
}

#[sqlx::test(fixtures(
    "../fixtures/projects.sql",
    "../fixtures/service_accounts.sql",
    "../fixtures/environments.sql",
    "../fixtures/project_access.sql"
))]
async fn test_project_access_repository_create_with_duplicate_fails(pool: PgPool) {
    let repository = ProjectAccessRepository::new(Arc::new(pool));
    let payload = ProjectAccessCreatePayload {
        project_id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        service_account_id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        environment_id: "00000000-0000-0000-0000-000000000001".to_string(),
        enabled: true,
    };
    let result = repository.create(payload).await;
    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();
    assert_eq!(error_message, "Project Id, Service Account Id and Environment Id combination already exists");
}

#[sqlx::test(fixtures(
    "../fixtures/projects.sql",
    "../fixtures/service_accounts.sql",
    "../fixtures/environments.sql",
    "../fixtures/project_access.sql"
))]
async fn test_project_access_repository_read_existing_succeeds(pool: PgPool) {
    let repository = ProjectAccessRepository::new(Arc::new(pool));
    let id = Uuid::parse_str("00000000-0000-0000-0000-000000000101").unwrap();
    let project_access = repository.read(id).await.unwrap();
    assert!(project_access.is_some());
    let project_access = project_access.unwrap();
    assert_eq!(project_access.id, Some(id));
    assert_eq!(project_access.project_id, Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap());
}

#[sqlx::test]
async fn test_project_access_repository_read_nonexistent_returns_error(pool: PgPool) {
    let repository = ProjectAccessRepository::new(Arc::new(pool));
    let id = Uuid::new_v4();
    let result = repository.read(id).await;
    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();
    assert_eq!(error_message, "Project access not found");
}

#[sqlx::test(fixtures(
    "../fixtures/projects.sql",
    "../fixtures/service_accounts.sql",
    "../fixtures/environments.sql",
    "../fixtures/project_access.sql"
))]
async fn test_project_access_repository_update_enabled_false_succeeds(pool: PgPool) {
    let repository = ProjectAccessRepository::new(Arc::new(pool));
    let id = Uuid::parse_str("00000000-0000-0000-0000-000000000101").unwrap();
    let update = ProjectAccessUpdatePayload {
        enabled: Some(false),
    };
    let project_access = repository.update(id, update).await.unwrap();
    assert!(!project_access.enabled);
}

#[sqlx::test(fixtures(
    "../fixtures/projects.sql",
    "../fixtures/service_accounts.sql",
    "../fixtures/environments.sql",
    "../fixtures/project_access.sql"
))]
async fn test_project_access_repository_update_enabled_true_succeeds(pool: PgPool) {
    let repository = ProjectAccessRepository::new(Arc::new(pool));
    let id = Uuid::parse_str("00000000-0000-0000-0000-000000000102").unwrap();
    let update = ProjectAccessUpdatePayload {
        enabled: Some(true),
    };
    let project_access = repository.update(id, update).await.unwrap();
    assert!(project_access.enabled);
}

#[sqlx::test(fixtures(
    "../fixtures/projects.sql",
    "../fixtures/service_accounts.sql",
    "../fixtures/environments.sql",
    "../fixtures/project_access.sql"
))]
async fn test_project_access_delete_existing_succeeds(pool: PgPool) {
    let repository = ProjectAccessRepository::new(Arc::new(pool));
    let id = Uuid::parse_str("00000000-0000-0000-0000-000000000101").unwrap();
    let deleted = repository.delete(id).await.unwrap();
    assert!(deleted);
}

#[sqlx::test]
async fn test_project_access_delete_nonexisting_fails(pool: PgPool) {
    let repository = ProjectAccessRepository::new(Arc::new(pool));
    let id = Uuid::new_v4();
    let result = repository.delete(id).await;
    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();
    assert_eq!(error_message, "Project access not found");
}

#[sqlx::test(fixtures(
    "../fixtures/projects.sql",
    "../fixtures/service_accounts.sql",
    "../fixtures/environments.sql",
    "../fixtures/project_access.sql"
))]
async fn test_project_access_find_with_limit_pagination(pool: PgPool) {
    let repository = ProjectAccessRepository::new(Arc::new(pool));
    let filter = ProjectAccessFilter::default();
    let sort = None;
    let pagination = Some(Pagination {
        limit: Some(2),
        offset: None,
    });
    let project_accesses = repository.find(filter, sort, pagination).await.unwrap();
    assert_eq!(project_accesses.len(), 2);
}

#[sqlx::test(fixtures(
    "../fixtures/projects.sql",
    "../fixtures/service_accounts.sql",
    "../fixtures/environments.sql",
    "../fixtures/project_access.sql"
))]
async fn test_project_access_find_with_offset_pagination(pool: PgPool) {
    let repository = ProjectAccessRepository::new(Arc::new(pool));
    let filter = ProjectAccessFilter::default();
    let sort = None;
    let pagination = Some(Pagination {
        limit: None,
        offset: Some(1),
    });
    let project_accesses = repository.find(filter, sort, pagination).await.unwrap();
    assert_eq!(project_accesses.len(), 3);
}

#[sqlx::test(fixtures(
    "../fixtures/projects.sql",
    "../fixtures/service_accounts.sql",
    "../fixtures/environments.sql",
    "../fixtures/project_access.sql"
))]
async fn test_project_access_find_with_limit_offset_pagination(pool: PgPool) {
    let repository = ProjectAccessRepository::new(Arc::new(pool));
    let filter = ProjectAccessFilter::default();
    let sort = None;
    let pagination = Some(Pagination {
        limit: Some(1),
        offset: Some(1),
    });
    let project_accesses = repository.find(filter, sort, pagination).await.unwrap();
    assert_eq!(project_accesses.len(), 1);
}

#[sqlx::test(fixtures(
    "../fixtures/projects.sql",
    "../fixtures/service_accounts.sql",
    "../fixtures/environments.sql",
    "../fixtures/project_access.sql"
))]
async fn test_project_access_find_with_project_id_filter(pool: PgPool) {
    let repository = ProjectAccessRepository::new(Arc::new(pool));
    let filter = ProjectAccessFilter {
        project_id: Some("123e4567-e89b-12d3-a456-426614174000".to_string()),
        ..Default::default()
    };
    let sort = None;
    let pagination = None;
    let project_accesses = repository.find(filter, sort, pagination).await.unwrap();
    assert_eq!(project_accesses.len(), 3);
}

#[sqlx::test(fixtures(
    "../fixtures/projects.sql",
    "../fixtures/service_accounts.sql",
    "../fixtures/environments.sql",
    "../fixtures/project_access.sql"
))]
async fn test_project_access_find_with_service_account_id_filter(pool: PgPool) {
    let repository = ProjectAccessRepository::new(Arc::new(pool));
    let filter = ProjectAccessFilter {
        service_account_id: Some("123e4567-e89b-12d3-a456-426614174000".to_string()),
        ..Default::default()
    };
    let sort = None;
    let pagination = None;
    let project_accesses = repository.find(filter, sort, pagination).await.unwrap();
    assert_eq!(project_accesses.len(), 1);
}

#[sqlx::test(fixtures(
    "../fixtures/projects.sql",
    "../fixtures/service_accounts.sql",
    "../fixtures/environments.sql",
    "../fixtures/project_access.sql"
))]
async fn test_project_access_find_with_environment_id_filter(pool: PgPool) {
    let repository = ProjectAccessRepository::new(Arc::new(pool));
    let filter = ProjectAccessFilter {
        environment_id: Some("00000000-0000-0000-0000-000000000001".to_string()),
        ..Default::default()
    };
    let sort = None;
    let pagination = None;
    let project_accesses = repository.find(filter, sort, pagination).await.unwrap();
    assert_eq!(project_accesses.len(), 1);
}

#[sqlx::test(fixtures(
    "../fixtures/projects.sql",
    "../fixtures/service_accounts.sql",
    "../fixtures/environments.sql",
    "../fixtures/project_access.sql"
))]
async fn test_project_access_find_with_enabled_filter(pool: PgPool) {
    let repository = ProjectAccessRepository::new(Arc::new(pool));
    let filter = ProjectAccessFilter {
        enabled: Some(false),
        ..Default::default()
    };
    let sort = None;
    let pagination = None;
    let project_accesses = repository.find(filter, sort, pagination).await.unwrap();
    assert_eq!(project_accesses.len(), 1);
}
