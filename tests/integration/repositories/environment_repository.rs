use std::sync::Arc;

use sentinel_guard::{models::environment::{EnvironmentCreatePayload, EnvironmentUpdatePayload}, repositories::{base::Repository, environment_repository::EnvironmentRepository}};
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


#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_repository_read_existing_account_succeeds(pool: PgPool) {
    let repository = EnvironmentRepository::new(Arc::new(pool));

    let response = repository.read("00000000-0000-0000-0000-000000000001".parse().unwrap()).await;

    assert!(response.is_ok());
    let environment = response.unwrap();

    assert!(environment.is_some());

    let environment = environment.unwrap();
    assert_eq!(environment.name, "dev");
    assert_eq!(environment.description, "Development environment");
    assert_eq!(environment.enabled, true);
}



#[sqlx::test]
async fn test_environment_repository_read_nonexistent_account_returns_error(pool: PgPool) {
    let repository = EnvironmentRepository::new(Arc::new(pool));

    let response = repository.read("00000000-0000-0000-0000-000000000002".parse().unwrap()).await;

    assert!(response.is_err());
    assert_eq!(response.unwrap_err().to_string(), "Environment not found");
}


#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_repository_update_name_succeeds(pool: PgPool) {
    let repository = EnvironmentRepository::new(Arc::new(pool));

    let response = repository.update("00000000-0000-0000-0000-000000000001".parse().unwrap(), EnvironmentUpdatePayload {
        name: Some("change-me".to_string()),
        description: None,
        enabled: None,
    }).await;

    assert!(response.is_ok());
    let environment = response.unwrap();

    assert_eq!(environment.name, "change-me");
}


#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_repository_update_description_succeeds(pool: PgPool) {
    let repository = EnvironmentRepository::new(Arc::new(pool));

    let response = repository.update("00000000-0000-0000-0000-000000000001".parse().unwrap(), EnvironmentUpdatePayload {
        name: None,
        description: Some("change-me".to_string()),
        enabled: None,
    }).await;

    assert!(response.is_ok());
    let environment = response.unwrap();

    assert_eq!(environment.description, "change-me");
}


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
