use std::sync::Arc;

use sentinel_guard::{models::environment::EnvironmentCreatePayload, repositories::{base::Repository, environment_repository::EnvironmentRepository}};
use sqlx::PgPool;

#[sqlx::test(fixtures("../fixtures/projects.sql"))]
async fn test_environment_repository_create_with_valid_data_succeeds(pool: PgPool) {
    let repository = EnvironmentRepository::new(Arc::new(pool));

    let payload = EnvironmentCreatePayload {
        project_id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        name: "test".to_string(),
        description: "test".to_string(),
        enabled: true,
    };

    let response = repository.create(payload).await;

    let result = response.unwrap();
    assert_eq!(result.name, "test");
    assert_eq!(result.description, "test");
    assert_eq!(result.enabled, true);
}

#[sqlx::test]
async fn test_environment_repository_create_with_missing_project_id_fails(pool: PgPool) {
    let repository = EnvironmentRepository::new(Arc::new(pool));

    let payload = EnvironmentCreatePayload {
        project_id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        name: "test".to_string(),
        description: "test".to_string(),
        enabled: true,
    };

    let response = repository.create(payload).await;

    assert!(response.is_err());
    assert_eq!(response.unwrap_err().to_string(), "Project not found");
}


#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_repository_create_with_duplicate_project_id_scope_fails(pool: PgPool) {
    let repository = EnvironmentRepository::new(Arc::new(pool));

    let payload = EnvironmentCreatePayload {
        project_id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        name: "dev".to_string(),
        description: "test".to_string(),
        enabled: true,
    };

    let response = repository.create(payload).await;

    assert!(response.is_err());
    assert_eq!(response.unwrap_err().to_string(), "Project Id, name combination already exists");
}


// TODO: test_environment_repository_read_existing_account_succeeds
// TODO: test_environment_repository_read_nonexistent_account_returns_error
// TODO: test_environment_repository_update_scope_succeeds
// TODO: test_environment_repository_update_description_succeeds
// TODO: test_environment_repository_update_enabled_to_false_succeeds
// TODO: test_environment_repository_update_enabled_to_true_succeeds
// TODO: test_environment_repository_update_scope_duplicated_fails
// TODO: test_environment_repository_delete_existing_scope_succeeds
// TODO: test_environment_repository_delete_nonexisting_scope_fails
// TODO: test_environment_repository_find_with_limit_pagination
// TODO: test_environment_repository_find_with_offset_pagination
// TODO: test_environment_repository_find_with_limit_offset_pagination
// TODO: test_environment_repository_find_with_project_id_filter
// TODO: test_environment_repository_find_with_scope_filter
// TODO: test_environment_repository_find_with_description_filter
// TODO: test_environment_repository_find_with_enabled_is_true_filter
// TODO: test_environment_repository_find_with_enabled_is_false_filter
