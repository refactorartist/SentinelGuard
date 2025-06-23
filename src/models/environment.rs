use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::sort::SortOrder;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Environment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub project_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct EnvironmentResponse {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: String,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub project_id: String,
    #[schema(example = "Production")]
    pub name: String,
    #[schema(example = "Production environment for the application")]
    pub description: Option<String>,
    #[schema(example = "true")]
    pub enabled: bool,
    #[schema(example = "2025-06-23T03:48:22.000Z")]
    pub created_at: String,
    #[schema(example = "2025-06-23T03:48:22.000Z")]
    pub updated_at: String,
}

impl From<Environment> for EnvironmentResponse {
    fn from(value: Environment) -> Self {
        Self {
            id: value.id.unwrap().to_string(),
            project_id: value.project_id.to_string(),
            name: value.name,
            description: value.description,
            enabled: value.enabled,
            created_at: value.created_at.to_string(),
            updated_at: value.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct EnvironmentFilter {
    pub project_id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct EnvironmentCreatePayload {
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct EnvironmentUpdatePayload {
    pub name: Option<String>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EnvironmentSortableFields {
    Id,
    ProjectId,
    Name,
    UpdatedAt,
    CreatedAt,
}

impl From<EnvironmentSortableFields> for String {
    fn from(value: EnvironmentSortableFields) -> Self {
        match value {
            EnvironmentSortableFields::Id => "id".to_string(),
            EnvironmentSortableFields::ProjectId => "project_id".to_string(),
            EnvironmentSortableFields::Name => "name".to_string(),
            EnvironmentSortableFields::UpdatedAt => "updated_at".to_string(),
            EnvironmentSortableFields::CreatedAt => "created_at".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnvironmentSortOrder {
    pub field: EnvironmentSortableFields,
    pub order: SortOrder,
}

impl EnvironmentSortOrder {
    pub fn new(field: EnvironmentSortableFields, order: SortOrder) -> Self {
        Self { field, order }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_default() {
        let environment = Environment::default();
        assert!(environment.id.is_none());
        assert_eq!(environment.project_id, Uuid::nil());
        assert_eq!(environment.name, "");
        assert!(environment.description.is_none());
        assert!(!environment.enabled);
    }

    #[test]
    fn test_environment_filter_default() {
        let filter = EnvironmentFilter::default();
        assert!(filter.project_id.is_none());
        assert!(filter.name.is_none());
        assert!(filter.description.is_none());
        assert!(filter.enabled.is_none());
    }

    #[test]
    fn test_environment_sortable_fields_to_string() {
        assert_eq!(String::from(EnvironmentSortableFields::Id), "id");
        assert_eq!(
            String::from(EnvironmentSortableFields::ProjectId),
            "project_id"
        );
        assert_eq!(String::from(EnvironmentSortableFields::Name), "name");
        assert_eq!(
            String::from(EnvironmentSortableFields::UpdatedAt),
            "updated_at"
        );
        assert_eq!(
            String::from(EnvironmentSortableFields::CreatedAt),
            "created_at"
        );
    }

    #[test]
    fn test_environment_sort_order_new() {
        let sort = EnvironmentSortOrder::new(EnvironmentSortableFields::Name, SortOrder::Asc);
        assert!(matches!(sort.field, EnvironmentSortableFields::Name));
        assert!(matches!(sort.order, SortOrder::Asc));
    }
}