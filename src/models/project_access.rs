use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::sort::SortOrder;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectAccess {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub project_id: Uuid,
    pub service_account_id: Uuid,
    pub environment_id: Uuid,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct ProjectAccessResponse {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: String,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub project_id: String,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub service_account_id: String,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub environment_id: String,
    #[schema(example = "true")]
    pub enabled: bool,
    #[schema(example = "2025-06-16T03:48:22.000Z")]
    pub created_at: String,
    #[schema(example = "2025-06-16T03:48:22.000Z")]
    pub updated_at: String,
}

impl From<ProjectAccess> for ProjectAccessResponse {
    fn from(value: ProjectAccess) -> Self {
        Self {
            id: value.id.unwrap().to_string(),
            project_id: value.project_id.to_string(),
            service_account_id: value.service_account_id.to_string(),
            environment_id: value.environment_id.to_string(),
            enabled: value.enabled,
            created_at: value.created_at.to_string(),
            updated_at: value.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct ProjectAccessFilter {
    pub project_id: Option<String>,
    pub service_account_id: Option<String>,
    pub environment_id: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ProjectAccessCreatePayload {
    pub project_id: String,
    pub service_account_id: String,
    pub environment_id: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ProjectAccessUpdatePayload {
    pub service_account_id: Option<String>,
    pub environment_id: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProjectAccessSortableFields {
    Id,
    ProjectId,
    ServiceAccountId,
    EnvironmentId,
    UpdatedAt,
    CreatedAt,
}

impl From<ProjectAccessSortableFields> for String {
    fn from(value: ProjectAccessSortableFields) -> Self {
        match value {
            ProjectAccessSortableFields::Id => "id".to_string(),
            ProjectAccessSortableFields::ProjectId => "project_id".to_string(),
            ProjectAccessSortableFields::ServiceAccountId => "service_account_id".to_string(),
            ProjectAccessSortableFields::EnvironmentId => "environment_id".to_string(),
            ProjectAccessSortableFields::UpdatedAt => "updated_at".to_string(),
            ProjectAccessSortableFields::CreatedAt => "created_at".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectAccessSortOrder {
    pub field: ProjectAccessSortableFields,
    pub order: SortOrder,
}

impl ProjectAccessSortOrder {
    pub fn new(field: ProjectAccessSortableFields, order: SortOrder) -> Self {
        Self { field, order }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_access_default() {
        let project_access = ProjectAccess::default();
        assert!(project_access.id.is_none());
        assert_eq!(project_access.project_id, Uuid::nil());
        assert_eq!(project_access.service_account_id, Uuid::nil());
        assert_eq!(project_access.environment_id, Uuid::nil());
        assert!(!project_access.enabled);
    }

    #[test]
    fn test_project_access_filter_default() {
        let filter = ProjectAccessFilter::default();
        assert!(filter.project_id.is_none());
        assert!(filter.service_account_id.is_none());
        assert!(filter.environment_id.is_none());
        assert!(filter.enabled.is_none());
    }

    #[test]
    fn test_project_access_sortable_fields_to_string() {
        assert_eq!(String::from(ProjectAccessSortableFields::Id), "id");
        assert_eq!(
            String::from(ProjectAccessSortableFields::ProjectId),
            "project_id"
        );
        assert_eq!(
            String::from(ProjectAccessSortableFields::ServiceAccountId),
            "service_account_id"
        );
        assert_eq!(
            String::from(ProjectAccessSortableFields::EnvironmentId),
            "environment_id"
        );
        assert_eq!(
            String::from(ProjectAccessSortableFields::UpdatedAt),
            "updated_at"
        );
        assert_eq!(
            String::from(ProjectAccessSortableFields::CreatedAt),
            "created_at"
        );
    }

    #[test]
    fn test_project_access_sort_order_new() {
        let sort = ProjectAccessSortOrder::new(ProjectAccessSortableFields::ProjectId, SortOrder::Asc);
        assert!(matches!(sort.field, ProjectAccessSortableFields::ProjectId));
        assert!(matches!(sort.order, SortOrder::Asc));
    }
}