use std::sync::Arc;

use sentinel_guard::{
    models::project_scope::{ProjectScopeCreatePayload, ProjectScopeUpdatePayload},
    repositories::{base::Repository, project_scope_repository::ProjectScopeRepository},
};
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test(fixtures("../fixtures/projects.sql"))]
async fn test_project_scope_repository_create_with_valid_data_succeeds(pool: PgPool) {
    let repository = ProjectScopeRepository::new(Arc::new(pool));

    let payload = ProjectScopeCreatePayload {
        project_id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        scope: "test:read".to_string(),
        description: "Test Description".to_string(),
        enabled: true,
    };

    let project_scope = repository.create(payload.clone()).await.unwrap();

    assert_eq!(
        project_scope.project_id,
        Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap()
    );
    assert_eq!(project_scope.scope, "test:read");
    assert_eq!(project_scope.description, "Test Description");
    assert!(project_scope.enabled);
}

#[sqlx::test]
async fn test_project_scope_repository_create_with_missing_project_id_fails(pool: PgPool) {
    let repository = ProjectScopeRepository::new(Arc::new(pool));

    let payload = ProjectScopeCreatePayload {
        project_id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        scope: "test:read".to_string(),
        description: "Test Description".to_string(),
        enabled: true,
    };

    let project_scope = repository.create(payload.clone()).await;

    assert!(project_scope.is_err());
    let error_message = project_scope.unwrap_err().to_string();
    assert_eq!(error_message, "Project not found");
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/project_scopes.sql"))]
async fn test_project_scope_repository_create_with_duplicate_project_id_scope_fails(pool: PgPool) {
    let repository = ProjectScopeRepository::new(Arc::new(pool));

    let payload = ProjectScopeCreatePayload {
        project_id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        scope: "testa:read".to_string(),
        description: "Test Description".to_string(),
        enabled: true,
    };

    let project_scope = repository.create(payload.clone()).await;

    assert!(project_scope.is_err());
    let error_message = project_scope.unwrap_err().to_string();
    assert_eq!(
        error_message,
        "Project Id, scope combination already exists"
    );
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/project_scopes.sql"))]
async fn test_project_scope_repository_read_existing_account_succeeds(pool: PgPool) {
    let repository = ProjectScopeRepository::new(Arc::new(pool));

    let project_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();

    let project_scope = repository.read(project_id).await.unwrap();

    assert!(project_scope.is_some());
    let project_scope = project_scope.unwrap();
    assert_eq!(
        project_scope.project_id,
        Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap()
    );
    assert_eq!(project_scope.scope, "testa:read");
    assert_eq!(project_scope.description, "Read access to testa project");
    assert!(project_scope.enabled);
}

#[sqlx::test]
async fn test_project_scope_repository_read_nonexistent_account_returns_error(pool: PgPool) {
    let repository = ProjectScopeRepository::new(Arc::new(pool));

    let project_id = Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap();

    let project_scope = repository.read(project_id).await;

    assert!(project_scope.is_err());
    let error_message = project_scope.unwrap_err().to_string();
    assert_eq!(error_message, "Project scope not found");
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/project_scopes.sql"))]
async fn test_project_scope_repository_update_scope_succeeds(pool: PgPool) {
    let repository = ProjectScopeRepository::new(Arc::new(pool));

    let project_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    let update = ProjectScopeUpdatePayload {
        scope: Some("testa:changes-made".to_string()),
        description: None,
        enabled: None,
    };

    let project_scope = repository.update(project_id, update).await.unwrap();

    assert_eq!(project_scope.scope, "testa:changes-made");
}


#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/project_scopes.sql"))]
async fn test_project_scope_repository_update_description_succeeds(pool: PgPool) {
    let repository = ProjectScopeRepository::new(Arc::new(pool));

    let project_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    let update = ProjectScopeUpdatePayload {
        scope: None,
        description: Some("some changes to make".to_string()),
        enabled: None,
    };

    let project_scope = repository.update(project_id, update).await.unwrap();

    assert_eq!(project_scope.description, "some changes to make");
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/project_scopes.sql"))]
async fn test_project_scope_repository_update_enabled_to_false_succeeds(pool: PgPool) {
    let repository = ProjectScopeRepository::new(Arc::new(pool));

    let project_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    let update = ProjectScopeUpdatePayload {
        scope: None,
        description: None,
        enabled: Some(false),
    };

    let project_scope = repository.update(project_id, update).await.unwrap();

    assert!(!project_scope.enabled);
}


#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/project_scopes.sql"))]
async fn test_project_scope_repository_update_enabled_to_true_succeeds(pool: PgPool) {
    let repository = ProjectScopeRepository::new(Arc::new(pool));

    let project_id = Uuid::parse_str("00000000-0000-0000-0000-000000000023").unwrap();
    let update = ProjectScopeUpdatePayload {
        scope: None,
        description: None,
        enabled: Some(true),
    };

    let project_scope = repository.update(project_id, update).await.unwrap();

    assert!(project_scope.enabled);
}


#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/project_scopes.sql"))]
async fn test_project_scope_repository_update_scope_duplicated_fails(pool: PgPool) {
    let repository = ProjectScopeRepository::new(Arc::new(pool));

    let project_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    let update = ProjectScopeUpdatePayload {
        scope: Some("testa:write".to_string()),
        description: None,
        enabled: None,
    };

    let project_scope = repository.update(project_id, update).await;

    assert!(project_scope.is_err());
    let error_message = project_scope.unwrap_err().to_string();
    assert_eq!(error_message, "Project Id, scope combination already exists");
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/project_scopes.sql"))]
async fn test_project_scope_delete_existing_scope_succeeds(pool: PgPool) {
    let repository = ProjectScopeRepository::new(Arc::new(pool));

    let project_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();

    let project_scope = repository.delete(project_id).await.unwrap();

    assert!(project_scope);
}


#[sqlx::test]
async fn test_project_scope_delete_nonexisting_scope_fails(pool: PgPool) {
    let repository = ProjectScopeRepository::new(Arc::new(pool));

    let project_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();

    let project_scope = repository.delete(project_id).await;

    assert!(project_scope.is_err());
    let error_message = project_scope.unwrap_err().to_string();
    assert_eq!(error_message, "Project scope not found");
}