use std::sync::Arc;

use sentinel_guard::{models::project_scope::ProjectScopeCreatePayload, repositories::{base::Repository, project_scope_repository::ProjectScopeRepository}};
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

    assert_eq!(project_scope.project_id, Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap());
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
    assert_eq!(error_message, "Project Id, scope combination already exists");
}

