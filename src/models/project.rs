use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::sort::SortOrder;

use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Project {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct ProjectResponse {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: String,
    #[schema(example = "Project Name")]
    pub name: String,
    #[schema(example = "Project Description")]
    pub description: String,
    #[schema(example = "true")]
    pub enabled: bool,
    #[schema(example = "2025-06-16T03:48:22.000Z")]
    pub created_at: String,
    #[schema(example = "2025-06-16T03:48:22.000Z")]
    pub updated_at: String,
}

impl From<Project> for ProjectResponse {
    fn from(value: Project) -> Self {
        Self {
            id: value.id.unwrap().to_string(),
            name: value.name,
            description: value.description,
            enabled: value.enabled,
            created_at: value.created_at.to_string(),
            updated_at: value.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct ProjectFilter {
    pub name: Option<String>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ProjectCreatePayload {
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ProjectUpdatePayload {
    pub name: Option<String>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProjectSortableFields {
    Id,
    Name,
    UpdatedAt,
    CreatedAt,
}

impl From<ProjectSortableFields> for String {
    fn from(value: ProjectSortableFields) -> Self {
        match value {
            ProjectSortableFields::Id => "id".to_string(),
            ProjectSortableFields::Name => "name".to_string(),
            ProjectSortableFields::UpdatedAt => "updated_at".to_string(),
            ProjectSortableFields::CreatedAt => "created_at".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectSortOrder {
    pub field: ProjectSortableFields,
    pub order: SortOrder,
}

impl ProjectSortOrder {
    pub fn new(field: ProjectSortableFields, order: SortOrder) -> Self {
        Self { field, order }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_default() {
        let project = Project::default();
        assert!(project.id.is_none());
        assert_eq!(project.name, "");
        assert_eq!(project.description, "");
        assert!(!project.enabled);
    }

    #[test]
    fn test_project_filter_default() {
        let filter = ProjectFilter::default();
        assert!(filter.name.is_none());
        assert!(filter.description.is_none());
        assert!(filter.enabled.is_none());
    }

    #[test]
    fn test_project_sortable_fields_to_string() {
        assert_eq!(String::from(ProjectSortableFields::Id), "id");
        assert_eq!(String::from(ProjectSortableFields::Name), "name");
        assert_eq!(String::from(ProjectSortableFields::UpdatedAt), "updated_at");
        assert_eq!(String::from(ProjectSortableFields::CreatedAt), "created_at");
    }

    #[test]
    fn test_project_sort_order_new() {
        let sort = ProjectSortOrder::new(ProjectSortableFields::Name, SortOrder::Asc);
        assert!(matches!(sort.field, ProjectSortableFields::Name));
        assert!(matches!(sort.order, SortOrder::Asc));
    }
}
