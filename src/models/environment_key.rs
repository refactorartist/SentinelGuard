use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use jsonwebtoken::Algorithm;

use crate::models::sort::SortOrder;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EnvironmentKey {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub environment_id: Uuid,
    #[serde(with = "crate::serializers::algorithm")]
    pub algorithm: Algorithm,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct EnvironmentKeyResponse {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: String,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub environment_id: String,
    #[schema(example = "HS256")]    
    pub algorithm: String,
    #[schema(example = "2025-06-16T03:48:22.000Z")]
    pub created_at: String,
    #[schema(example = "2025-06-16T03:48:22.000Z")]
    pub updated_at: String,
}

impl From<EnvironmentKey> for EnvironmentKeyResponse {
    fn from(value: EnvironmentKey) -> Self {
        Self {
            id: value.id.unwrap().to_string(),
            environment_id: value.environment_id.to_string(),
            algorithm: format!("{:?}", value.algorithm),
            created_at: value.created_at.to_string(),
            updated_at: value.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct EnvironmentKeyFilter {
    pub environment_id: Option<String>,
    pub algorithm: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct EnvironmentKeyCreatePayload {
    pub environment_id: String,
    pub algorithm: String,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct EnvironmentKeyUpdatePayload {
    pub key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EnvironmentKeySortableFields {
    Id,
    EnvironmentId,
    Algorithm,
    CreatedAt,
    UpdatedAt,
}

impl From<EnvironmentKeySortableFields> for String {
    fn from(value: EnvironmentKeySortableFields) -> Self {
        match value {
            EnvironmentKeySortableFields::Id => "id".to_string(),
            EnvironmentKeySortableFields::EnvironmentId => "environment_id".to_string(),
            EnvironmentKeySortableFields::Algorithm => "algorithm".to_string(),
            EnvironmentKeySortableFields::CreatedAt => "created_at".to_string(),
            EnvironmentKeySortableFields::UpdatedAt => "updated_at".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnvironmentKeySortOrder {
    pub field: EnvironmentKeySortableFields,
    pub order: SortOrder,
}

impl EnvironmentKeySortOrder {
    pub fn new(field: EnvironmentKeySortableFields, order: SortOrder) -> Self {
        Self { field, order }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_environment_key_default() {
        let key = EnvironmentKey::default();
        assert!(key.id.is_none());
        assert_eq!(key.environment_id, Uuid::nil());
        assert_eq!(key.algorithm, Algorithm::HS256);
    }

    #[test]
    fn test_environment_key_filter_default() {
        let filter = EnvironmentKeyFilter::default();
        assert!(filter.environment_id.is_none());
        assert!(filter.algorithm.is_none());
    }

    #[test]
    fn test_environment_key_sortable_fields_to_string() {
        assert_eq!(String::from(EnvironmentKeySortableFields::Id), "id");
        assert_eq!(String::from(EnvironmentKeySortableFields::EnvironmentId), "environment_id");
        assert_eq!(String::from(EnvironmentKeySortableFields::Algorithm), "algorithm");
        assert_eq!(String::from(EnvironmentKeySortableFields::CreatedAt), "created_at");
        assert_eq!(String::from(EnvironmentKeySortableFields::UpdatedAt), "updated_at");
    }

    #[test]
    fn test_environment_key_sort_order_new() {
        let sort = EnvironmentKeySortOrder::new(EnvironmentKeySortableFields::Algorithm, SortOrder::Asc);
        assert!(matches!(sort.field, EnvironmentKeySortableFields::Algorithm));
        assert!(matches!(sort.order, SortOrder::Asc));
    }
} 