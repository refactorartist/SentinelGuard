use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::sort::SortOrder;

use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ServiceAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub name: String,
    pub email: String,
    pub secret: String,
    pub description: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct ServiceAccountResponse {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: String,
    #[schema(example = "Service Account Name")]
    pub name: String,
    #[schema(example = "service@example.com")]
    pub email: String,
    #[schema(example = "supersecretvalue")]
    pub secret: String,
    #[schema(example = "Service Account Description")]
    pub description: String,
    #[schema(example = "true")]
    pub enabled: bool,
    #[schema(example = "2025-06-16T03:48:22.000Z")]
    pub created_at: String,
    #[schema(example = "2025-06-16T03:48:22.000Z")]
    pub updated_at: String,
}

impl From<ServiceAccount> for ServiceAccountResponse {
    fn from(value: ServiceAccount) -> Self {
        Self {
            id: value.id.unwrap().to_string(),
            name: value.name,
            email: value.email,
            secret: value.secret,
            description: value.description,
            enabled: value.enabled,
            created_at: value.created_at.to_string(),
            updated_at: value.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct ServiceAccountFilter {
    pub name: Option<String>,
    pub email: Option<String>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ServiceAccountCreatePayload {
    pub name: String,
    pub email: String,
    pub secret: String,
    pub description: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ServiceAccountUpdatePayload {
    pub name: Option<String>,
    pub email: Option<String>,
    pub secret: Option<String>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServiceAccountSortableFields {
    Id,
    Name,
    Email,
    UpdatedAt,
    CreatedAt,
}

impl From<ServiceAccountSortableFields> for String {
    fn from(value: ServiceAccountSortableFields) -> Self {
        match value {
            ServiceAccountSortableFields::Id => "id".to_string(),
            ServiceAccountSortableFields::Name => "name".to_string(),
            ServiceAccountSortableFields::Email => "email".to_string(),
            ServiceAccountSortableFields::UpdatedAt => "updated_at".to_string(),
            ServiceAccountSortableFields::CreatedAt => "created_at".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceAccountSortOrder {
    pub field: ServiceAccountSortableFields,
    pub order: SortOrder,
}

impl ServiceAccountSortOrder {
    pub fn new(field: ServiceAccountSortableFields, order: SortOrder) -> Self {
        Self { field, order }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_account_default() {
        let sa = ServiceAccount::default();
        assert!(sa.id.is_none());
        assert_eq!(sa.name, "");
        assert_eq!(sa.email, "");
        assert_eq!(sa.secret, "");
        assert_eq!(sa.description, "");
        assert!(!sa.enabled);
    }

    #[test]
    fn test_service_account_filter_default() {
        let filter = ServiceAccountFilter::default();
        assert!(filter.name.is_none());
        assert!(filter.email.is_none());
        assert!(filter.description.is_none());
        assert!(filter.enabled.is_none());
    }

    #[test]
    fn test_service_account_sortable_fields_to_string() {
        assert_eq!(String::from(ServiceAccountSortableFields::Id), "id");
        assert_eq!(String::from(ServiceAccountSortableFields::Name), "name");
        assert_eq!(String::from(ServiceAccountSortableFields::Email), "email");
        assert_eq!(
            String::from(ServiceAccountSortableFields::UpdatedAt),
            "updated_at"
        );
        assert_eq!(
            String::from(ServiceAccountSortableFields::CreatedAt),
            "created_at"
        );
    }

    #[test]
    fn test_service_account_sort_order_new() {
        let sort = ServiceAccountSortOrder::new(ServiceAccountSortableFields::Name, SortOrder::Asc);
        assert!(matches!(sort.field, ServiceAccountSortableFields::Name));
        assert!(matches!(sort.order, SortOrder::Asc));
    }
}
