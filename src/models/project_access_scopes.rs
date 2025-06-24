use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::sort::SortOrder;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectAccessScope {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub project_access_id: Uuid,
    pub scope_id: Uuid,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct ProjectAccessScopeResponse {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: String,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub project_access_id: String,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub scope_id: String,
    #[schema(example = "true")]
    pub enabled: bool,
    #[schema(example = "2025-06-16T03:48:22.000Z")]
    pub created_at: String,
    #[schema(example = "2025-06-16T03:48:22.000Z")]
    pub updated_at: String,
}

impl From<ProjectAccessScope> for ProjectAccessScopeResponse {
    fn from(value: ProjectAccessScope) -> Self {
        Self {
            id: value.id.unwrap().to_string(),
            project_access_id: value.project_access_id.to_string(),
            scope_id: value.scope_id.to_string(),
            enabled: value.enabled,
            created_at: value.created_at.to_string(),
            updated_at: value.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct ProjectAccessScopeFilter {
    pub project_access_id: Option<String>,
    pub scope_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ProjectAccessScopeCreatePayload {
    pub project_access_id: String,
    pub scope_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ProjectAccessScopeUpdatePayload {
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProjectAccessScopeSortableFields {
    Id,
    ProjectAccessId,
    ScopeId,
    UpdatedAt,
    CreatedAt,
}

impl From<ProjectAccessScopeSortableFields> for String {
    fn from(value: ProjectAccessScopeSortableFields) -> Self {
        match value {
            ProjectAccessScopeSortableFields::Id => "id".to_string(),
            ProjectAccessScopeSortableFields::ProjectAccessId => "project_access_id".to_string(),
            ProjectAccessScopeSortableFields::ScopeId => "scope_id".to_string(),
            ProjectAccessScopeSortableFields::UpdatedAt => "updated_at".to_string(),
            ProjectAccessScopeSortableFields::CreatedAt => "created_at".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectAccessScopeSortOrder {
    pub field: ProjectAccessScopeSortableFields,
    pub order: SortOrder,
}

impl ProjectAccessScopeSortOrder {
    pub fn new(field: ProjectAccessScopeSortableFields, order: SortOrder) -> Self {
        Self { field, order }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_access_scope_default() {
        let project_access_scope = ProjectAccessScope::default();
        assert!(project_access_scope.id.is_none());
        assert_eq!(project_access_scope.project_access_id, Uuid::nil());
        assert_eq!(project_access_scope.scope_id, Uuid::nil());
    }

    #[test]
    fn test_project_access_scope_filter_default() {
        let filter = ProjectAccessScopeFilter::default();
        assert!(filter.project_access_id.is_none());
        assert!(filter.scope_id.is_none());
    }

    #[test]
    fn test_project_access_scope_sortable_fields_to_string() {
        assert_eq!(String::from(ProjectAccessScopeSortableFields::Id), "id");
        assert_eq!(
            String::from(ProjectAccessScopeSortableFields::ProjectAccessId),
            "project_access_id"
        );
        assert_eq!(
            String::from(ProjectAccessScopeSortableFields::ScopeId),
            "scope_id"
        );
        assert_eq!(
            String::from(ProjectAccessScopeSortableFields::UpdatedAt),
            "updated_at"
        );
        assert_eq!(
            String::from(ProjectAccessScopeSortableFields::CreatedAt),
            "created_at"
        );
    }

    #[test]
    fn test_project_access_scope_sort_order_new() {
        let sort = ProjectAccessScopeSortOrder::new(
            ProjectAccessScopeSortableFields::ScopeId,
            SortOrder::Asc,
        );
        assert!(matches!(
            sort.field,
            ProjectAccessScopeSortableFields::ScopeId
        ));
        assert!(matches!(sort.order, SortOrder::Asc));
    }
}
