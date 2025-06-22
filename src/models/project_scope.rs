use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::sort::SortOrder;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectScope {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub project_id: Uuid,
    pub scope: String,
    pub description: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct ProjectScopeResponse {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: String,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub project_id: String,
    #[schema(example = "Project Scope Name")]
    pub scope: String,
    #[schema(example = "Project Scope Description")]
    pub description: String,
    #[schema(example = "true")]
    pub enabled: bool,
    #[schema(example = "2025-06-16T03:48:22.000Z")]
    pub created_at: String,
    #[schema(example = "2025-06-16T03:48:22.000Z")]
    pub updated_at: String,
}

impl From<ProjectScope> for ProjectScopeResponse {
    fn from(value: ProjectScope) -> Self {
        Self {
            id: value.id.unwrap().to_string(),
            project_id: value.project_id.to_string(),
            scope: value.scope,
            description: value.description,
            enabled: value.enabled,
            created_at: value.created_at.to_string(),
            updated_at: value.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct ProjectScopeFilter {
    pub project_id: Option<String>,
    pub scope: Option<String>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ProjectScopeCreatePayload {
    pub project_id: String,
    pub scope: String,
    pub description: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ProjectScopeUpdatePayload {
    pub scope: Option<String>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProjectScopeSortableFields {
    Id,
    ProjectId,
    Scope,
    UpdatedAt,
    CreatedAt,
}

impl From<ProjectScopeSortableFields> for String {
    fn from(value: ProjectScopeSortableFields) -> Self {
        match value {
            ProjectScopeSortableFields::Id => "id".to_string(),
            ProjectScopeSortableFields::ProjectId => "project_id".to_string(),
            ProjectScopeSortableFields::Scope => "scope".to_string(),
            ProjectScopeSortableFields::UpdatedAt => "updated_at".to_string(),
            ProjectScopeSortableFields::CreatedAt => "created_at".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectScopeSortOrder {
    pub field: ProjectScopeSortableFields,
    pub order: SortOrder,
}

impl ProjectScopeSortOrder {
    pub fn new(field: ProjectScopeSortableFields, order: SortOrder) -> Self {
        Self { field, order }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_scope_default() {
        let project_scope = ProjectScope::default();
        assert!(project_scope.id.is_none());
        assert_eq!(project_scope.project_id, Uuid::nil());
        assert_eq!(project_scope.scope, "");
        assert_eq!(project_scope.description, "");
        assert!(!project_scope.enabled);
    }

    #[test]
    fn test_project_scope_filter_default() {
        let filter = ProjectScopeFilter::default();
        assert!(filter.project_id.is_none());
        assert!(filter.scope.is_none());
        assert!(filter.description.is_none());
        assert!(filter.enabled.is_none());
    }

    #[test]
    fn test_project_scope_sortable_fields_to_string() {
        assert_eq!(String::from(ProjectScopeSortableFields::Id), "id");
        assert_eq!(
            String::from(ProjectScopeSortableFields::ProjectId),
            "project_id"
        );
        assert_eq!(String::from(ProjectScopeSortableFields::Scope), "scope");
        assert_eq!(
            String::from(ProjectScopeSortableFields::UpdatedAt),
            "updated_at"
        );
        assert_eq!(
            String::from(ProjectScopeSortableFields::CreatedAt),
            "created_at"
        );
    }

    #[test]
    fn test_project_scope_sort_order_new() {
        let sort = ProjectScopeSortOrder::new(ProjectScopeSortableFields::Scope, SortOrder::Asc);
        assert!(matches!(sort.field, ProjectScopeSortableFields::Scope));
        assert!(matches!(sort.order, SortOrder::Asc));
    }
}
