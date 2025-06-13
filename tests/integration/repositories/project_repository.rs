use std::sync::Arc;

use sentinel_guard::{
    models::{
        pagination::Pagination,
        project::{
            ProjectCreatePayload, ProjectFilter, ProjectSortOrder, ProjectSortableFields,
            ProjectUpdatePayload,
        },
        sort::SortOrder,
    },
    repositories::{base::Repository, project_repository::ProjectRepository},
};
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test]
async fn test_project_repository_create_project_succeeds(pool: PgPool) {
    let project_repository = ProjectRepository::new(Arc::new(pool));

    let payload = ProjectCreatePayload {
        name: "test".to_string(),
        description: "test".to_string(),
        enabled: true,
    };

    let project = project_repository.create(payload).await.unwrap();

    assert_eq!(project.name, "test");
    assert_eq!(project.description, "test");
    assert!(project.enabled);
}

#[sqlx::test]
async fn test_project_repository_create_duplicate_error(pool: PgPool) {
    let project_repository = ProjectRepository::new(Arc::new(pool));

    let payload = ProjectCreatePayload {
        name: "test".to_string(),
        description: "description".to_string(),
        enabled: true,
    };

    // Add a record with payload
    let _ = project_repository.create(payload.clone()).await;

    // Add another record with the same name
    let project = project_repository.create(payload.clone()).await;

    dbg!(&project);

    assert!(project.is_err());
}

#[sqlx::test(fixtures("projects"))]
async fn test_project_repository_read_success(pool: PgPool) {
    let project_repository = ProjectRepository::new(Arc::new(pool));

    let project = project_repository
        .read(Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap())
        .await
        .unwrap();

    assert!(project.is_some());

    let project = project.unwrap();

    assert_eq!(project.name, "testa");
    assert_eq!(project.description, "test");
    assert!(project.enabled);
}

#[sqlx::test]
async fn test_project_repository_read_not_found(pool: PgPool) {
    let project_repository = ProjectRepository::new(Arc::new(pool));

    let project = project_repository
        .read(Uuid::parse_str("123e4567-e89b-12d3-a456-426614174002").unwrap())
        .await;

    assert!(project.is_err());

    assert_eq!(project.unwrap_err().to_string(), "Project not found");
}

#[sqlx::test(fixtures("projects"))]
async fn test_project_repository_update_name_success(pool: PgPool) {
    test_project_repository_update_success(
        pool,
        ProjectUpdatePayload {
            name: Some("test1".to_string()),
            description: None,
            enabled: None,
        },
        |project| assert_eq!(project.name, "test1"),
    )
    .await;
}

#[sqlx::test(fixtures("projects"))]
async fn test_project_repository_update_description_success(pool: PgPool) {
    test_project_repository_update_success(
        pool,
        ProjectUpdatePayload {
            name: None,
            description: Some("test1".to_string()),
            enabled: None,
        },
        |project| assert_eq!(project.description, "test1"),
    )
    .await;
}

#[sqlx::test(fixtures("projects"))]
async fn test_project_repository_update_enabled_false_success(pool: PgPool) {
    test_project_repository_update_success(
        pool,
        ProjectUpdatePayload {
            name: None,
            description: None,
            enabled: Some(false),
        },
        |project| assert!(!project.enabled),
    )
    .await;
}

#[sqlx::test(fixtures("projects"))]
async fn test_project_repository_update_enabled_true_success(pool: PgPool) {
    test_project_repository_update_success(
        pool,
        ProjectUpdatePayload {
            name: None,
            description: None,
            enabled: Some(true),
        },
        |project| assert!(project.enabled),
    )
    .await;
}

async fn test_project_repository_update_success<F>(
    pool: PgPool,
    payload: ProjectUpdatePayload,
    assertion: F,
) where
    F: FnOnce(&sentinel_guard::models::project::Project),
{
    // Create a new project repository with the provided connection pool
    let project_repository = ProjectRepository::new(Arc::new(pool));

    // Update a project with a specific UUID using the provided payload
    // This UUID must match an existing project in the fixtures
    let project = project_repository
        .update(
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap(),
            payload,
        )
        .await
        .unwrap();

    // Execute the provided assertion function on the updated project
    // This allows each test case to verify different aspects of the update
    assertion(&project);
}

#[sqlx::test]
async fn test_project_repository_update_no_changes_were_made_error(pool: PgPool) {
    let project_repository = ProjectRepository::new(Arc::new(pool));

    let project = project_repository
        .update(
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174002").unwrap(),
            ProjectUpdatePayload {
                name: Some("tested".to_string()),
                description: None,
                enabled: None,
            },
        )
        .await;

    assert!(project.is_err());
    assert_eq!(project.unwrap_err().to_string(), "No changes were made");
}

#[sqlx::test(fixtures("projects"))]
async fn test_project_repository_delete_success(pool: PgPool) {
    let project_repository = ProjectRepository::new(Arc::new(pool));

    let is_deleted = project_repository
        .delete(Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap())
        .await
        .unwrap();

    assert!(is_deleted);
}

#[sqlx::test]
async fn test_project_repository_delete_nothing_to_delete(pool: PgPool) {
    let project_repository = ProjectRepository::new(Arc::new(pool));

    let is_deleted = project_repository
        .delete(Uuid::parse_str("123e4567-e89b-12d3-a456-426614174002").unwrap())
        .await
        .unwrap();

    assert!(!is_deleted);
}

#[sqlx::test(fixtures("projects"))]
async fn test_project_repository_find_no_filters(pool: PgPool) {
    let project_repository = ProjectRepository::new(Arc::new(pool));

    let filter = ProjectFilter::default();
    let sort = None;
    let pagination = None;

    let projects = project_repository
        .find(filter, sort, pagination)
        .await
        .unwrap();

    // Should be equal to number of records in ./fixtures/projects.sql
    assert_eq!(projects.len(), 4);
}

#[sqlx::test(fixtures("projects"))]
async fn test_project_repository_find_filter_name_success(pool: PgPool) {
    let project_repository = ProjectRepository::new(Arc::new(pool));

    let filter = ProjectFilter {
        name: Some("test".to_string()),
        ..Default::default()
    };
    let sort = None;
    let pagination = None;

    let projects = project_repository
        .find(filter, sort, pagination)
        .await
        .unwrap();

    assert_eq!(projects.len(), 2);
}

#[sqlx::test(fixtures("projects"))]
async fn test_project_repository_find_filter_description_success(pool: PgPool) {
    let project_repository = ProjectRepository::new(Arc::new(pool));

    let filter = ProjectFilter {
        description: Some("test".to_string()),
        ..Default::default()
    };
    let sort = None;
    let pagination = None;

    let projects = project_repository
        .find(filter, sort, pagination)
        .await
        .unwrap();

    assert_eq!(projects.len(), 2);
}

#[sqlx::test(fixtures("projects"))]
async fn test_project_repository_find_filter_enabled_true_success(pool: PgPool) {
    let project_repository = ProjectRepository::new(Arc::new(pool));

    let filter = ProjectFilter {
        enabled: Some(true),
        ..Default::default()
    };
    let sort = None;
    let pagination = None;

    let projects = project_repository
        .find(filter, sort, pagination)
        .await
        .unwrap();

    assert_eq!(projects.len(), 3);
}

#[sqlx::test(fixtures("projects"))]
async fn test_project_repository_find_filter_enabled_false_success(pool: PgPool) {
    let project_repository = ProjectRepository::new(Arc::new(pool));

    let filter = ProjectFilter {
        enabled: Some(false),
        ..Default::default()
    };
    let sort = None;
    let pagination = None;

    let projects = project_repository
        .find(filter, sort, pagination)
        .await
        .unwrap();

    assert_eq!(projects.len(), 1);
}

#[sqlx::test]
async fn test_project_repository_find_no_filters_no_records(pool: PgPool) {
    let project_repository = ProjectRepository::new(Arc::new(pool));

    let filter = ProjectFilter::default();
    let sort = None;
    let pagination = None;

    let projects = project_repository
        .find(filter, sort, pagination)
        .await
        .unwrap();

    assert_eq!(projects.len(), 0);
}

#[sqlx::test(fixtures("projects"))]
async fn test_project_repository_find_pagination_success_offset(pool: PgPool) {
    let pagination = Pagination {
        limit: None,
        offset: Some(2),
    };

    test_project_repository_find_pagination_success(pool, pagination, |projects| {
        assert_eq!(projects.len(), 2)
    })
    .await;
}

#[sqlx::test(fixtures("projects"))]
async fn test_project_repository_find_pagination_success_limit(pool: PgPool) {
    let pagination = Pagination {
        limit: Some(2),
        offset: None,
    };

    test_project_repository_find_pagination_success(pool, pagination, |projects| {
        assert_eq!(projects.len(), 2)
    })
    .await;
}

async fn test_project_repository_find_pagination_success<F>(
    pool: PgPool,
    pagination: Pagination,
    assertion: F,
) where
    F: FnOnce(&Vec<sentinel_guard::models::project::Project>),
{
    // Create a new project repository with the provided connection pool
    let project_repository = ProjectRepository::new(Arc::new(pool));

    // Set up default filter with no specific conditions
    let filter = ProjectFilter::default();
    // No sorting specified
    let sort = None;
    // Apply the pagination configuration provided by the test case
    let pagination = Some(pagination);

    // Retrieve projects with the specified pagination settings
    let projects = project_repository
        .find(filter, sort, pagination)
        .await
        .unwrap();

    // Execute the provided assertion function on the retrieved projects
    // This allows each test case to verify different aspects of pagination
    assertion(&projects);
}

#[sqlx::test(fixtures("sort_projects"))]
async fn test_project_repository_find_sort_name_asc_success(pool: PgPool) {
    let project_repository = ProjectRepository::new(Arc::new(pool));

    let filter = ProjectFilter::default();
    let sort = Some(vec![ProjectSortOrder::new(
        ProjectSortableFields::Name,
        SortOrder::Asc,
    )]);
    let pagination = None;

    let projects = project_repository
        .find(filter, sort, pagination)
        .await
        .unwrap();

    assert_eq!(projects.len(), 6);
    assert_eq!(projects[0].name, "testa");
    assert_eq!(projects[1].name, "testb");
    assert_eq!(projects[2].name, "testc");
    assert_eq!(projects[3].name, "testd");
    assert_eq!(projects[4].name, "teste");
    assert_eq!(projects[5].name, "testf");
}

#[sqlx::test(fixtures("sort_projects"))]
async fn test_project_repository_find_sort_name_desc_success(pool: PgPool) {
    let project_repository = ProjectRepository::new(Arc::new(pool));

    let filter = ProjectFilter::default();
    let sort = Some(vec![ProjectSortOrder::new(
        ProjectSortableFields::Name,
        SortOrder::Desc,
    )]);
    let pagination = None;

    let projects = project_repository
        .find(filter, sort, pagination)
        .await
        .unwrap();

    assert_eq!(projects.len(), 6);
    assert_eq!(projects[0].name, "testf");
    assert_eq!(projects[1].name, "teste");
    assert_eq!(projects[2].name, "testd");
    assert_eq!(projects[3].name, "testc");
    assert_eq!(projects[4].name, "testb");
    assert_eq!(projects[5].name, "testa");
}

#[sqlx::test(fixtures("sort_projects"))]
async fn test_project_repository_find_sort_id_asc_success(pool: PgPool) {
    let project_repository = ProjectRepository::new(Arc::new(pool));

    let filter = ProjectFilter::default();
    let sort = Some(vec![ProjectSortOrder::new(
        ProjectSortableFields::Id,
        SortOrder::Asc,
    )]);
    let pagination = None;

    let projects = project_repository
        .find(filter, sort, pagination)
        .await
        .unwrap();

    assert_eq!(projects.len(), 6);
    assert_eq!(projects[0].name, "testa");
    assert_eq!(projects[1].name, "testb");
    assert_eq!(projects[2].name, "testc");
    assert_eq!(projects[3].name, "testd");
    assert_eq!(projects[4].name, "teste");
    assert_eq!(projects[5].name, "testf");
}

#[sqlx::test(fixtures("sort_projects"))]
async fn test_project_repository_find_sort_id_desc_success(pool: PgPool) {
    let project_repository = ProjectRepository::new(Arc::new(pool));

    let filter = ProjectFilter::default();
    let sort = Some(vec![ProjectSortOrder::new(
        ProjectSortableFields::Id,
        SortOrder::Desc,
    )]);
    let pagination = None;

    let projects = project_repository
        .find(filter, sort, pagination)
        .await
        .unwrap();

    assert_eq!(projects.len(), 6);
    assert_eq!(projects[0].name, "testf");
    assert_eq!(projects[1].name, "teste");
    assert_eq!(projects[2].name, "testd");
    assert_eq!(projects[3].name, "testc");
    assert_eq!(projects[4].name, "testb");
    assert_eq!(projects[5].name, "testa");
}
