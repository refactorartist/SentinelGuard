use std::sync::Arc;

use sentinel_guard::{models::{environment::{EnvironmentCreatePayload, EnvironmentFilter, EnvironmentUpdatePayload}, pagination::Pagination}, repositories::{base::Repository, environment_repository::EnvironmentRepository}};
use sqlx::PgPool;
use uuid::Uuid;

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


#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_repository_update_enabled_to_false_succeeds(pool: PgPool) {
    let repository = EnvironmentRepository::new(Arc::new(pool));

    let response = repository.update("00000000-0000-0000-0000-000000000001".parse().unwrap(), EnvironmentUpdatePayload {
        name: None,
        description: None,
        enabled: Some(false),
    }).await;

    assert!(response.is_ok());
    let environment = response.unwrap();

    assert_eq!(environment.enabled, false);
}


#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_repository_update_enabled_to_true_succeeds(pool: PgPool) {
    let repository = EnvironmentRepository::new(Arc::new(pool));

    let response = repository.update("00000000-0000-0000-0000-000000000002".parse().unwrap(), EnvironmentUpdatePayload {
        name: None,
        description: None,
        enabled: Some(true),
    }).await;

    assert!(response.is_ok());
    let environment = response.unwrap();

    assert_eq!(environment.enabled, true);
}


#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_repository_update_scope_duplicated_fails(pool: PgPool) {
    let repository = EnvironmentRepository::new(Arc::new(pool));

    let response = repository.update("00000000-0000-0000-0000-000000000002".parse().unwrap(), EnvironmentUpdatePayload {
        name: Some("dev".to_string()),
        description: None,
        enabled: None,
    }).await;

    assert!(response.is_err());
    assert_eq!(response.unwrap_err().to_string(), "Project Id, name combination already exists");
}


#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_repository_delete_existing_environment_succeeds(pool: PgPool) {
    let repository = EnvironmentRepository::new(Arc::new(pool));

    let response = repository.delete("00000000-0000-0000-0000-000000000001".parse().unwrap()).await;

    assert!(response.is_ok());
}


#[sqlx::test(fixtures("../fixtures/projects.sql"))]
async fn test_environment_repository_delete_nonexisting_environment_fails(pool: PgPool) {
    let repository = EnvironmentRepository::new(Arc::new(pool));

    let response = repository.delete("00000000-0000-0000-0000-000000000002".parse().unwrap()).await;

    assert!(response.is_err());
    assert_eq!(response.unwrap_err().to_string(), "Environment not found");
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_repository_find_with_limit_pagination(pool: PgPool) {
    let repository = EnvironmentRepository::new(Arc::new(pool));
    let pagination = Pagination {
        limit: Some(1),
        offset: Some(0),
    };

    let filter = EnvironmentFilter::default();

    let sort = None;

    let response = repository.find(filter, sort, Some(pagination)).await;

    assert!(response.is_ok());
    let environments = response.unwrap();

    assert_eq!(environments.len(), 1);
}

#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_repository_find_with_offset_pagination(pool: PgPool) {
    let repository = EnvironmentRepository::new(Arc::new(pool));
    
    // First request - get first item
    let first_pagination = Pagination {
        limit: Some(1),
        offset: None,
    };
    
    let first_response = repository.find(EnvironmentFilter::default(), None, Some(first_pagination)).await;
    assert!(first_response.is_ok());
    let first_environments = first_response.unwrap();
    assert_eq!(first_environments.len(), 1);
    
    // Second request - get first item with explicit offset
    let second_pagination = Pagination {
        limit: Some(1),
        offset: Some(1),
    };
    
    let second_response = repository.find(EnvironmentFilter::default(), None, Some(second_pagination)).await;
    assert!(second_response.is_ok());
    let second_environments = second_response.unwrap();
    assert_eq!(second_environments.len(), 1);
    
    // Verify we got the same environment (since offset 0 is the same as no offset)
    assert_ne!(first_environments[0].id, second_environments[0].id);
}

// TODO: test_environment_repository_find_with_project_id_filter
#[sqlx::test(fixtures("../fixtures/projects.sql", "../fixtures/environments.sql"))]
async fn test_environment_repository_find_with_project_id_filter(pool: PgPool) {
    let repository = EnvironmentRepository::new(Arc::new(pool));

    let project_id = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap();

    let filter = EnvironmentFilter {
        project_id: Some(project_id.to_string()),
        ..Default::default()
    };
    let sort = None;
    let pagination = None;

    let environments = repository.find(filter, sort, pagination).await.unwrap();

    for environment in &environments {
        assert_eq!(environment.project_id, project_id);
    }
}

// TODO: test_environment_repository_find_with_scope_filter
// TODO: test_environment_repository_find_with_description_filter
// TODO: test_environment_repository_find_with_enabled_is_true_filter
// TODO: test_environment_repository_find_with_enabled_is_false_filter
