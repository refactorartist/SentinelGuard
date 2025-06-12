use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::models::sort::SortOrder;


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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectFilter {
    pub name: Option<String>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectCreatePayload {
    pub name: String,
    pub description: String,
    pub enabled: bool,
}
    

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    