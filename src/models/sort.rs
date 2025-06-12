use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sort {
    pub field: String,
    pub order: SortOrder,
}

impl Sort {
    pub fn new(field: String, order: SortOrder) -> Self {
        Self { field, order }
    }
    
    pub fn order(&self) -> &SortOrder {
        &self.order
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SortOrder {
    Asc,
    Desc,
}
    

impl std::fmt::Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortOrder::Asc => write!(f, "ASC"),
            SortOrder::Desc => write!(f, "DESC"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_new() {
        let sort = Sort::new("name".to_string(), SortOrder::Asc);
        assert_eq!(sort.field, "name");
        assert!(matches!(sort.order, SortOrder::Asc));
    }

    #[test]
    fn test_sort_order() {
        let sort = Sort::new("id".to_string(), SortOrder::Desc);
        assert!(matches!(*sort.order(), SortOrder::Desc));
    }

    #[test]
    fn test_sort_order_display() {
        assert_eq!(SortOrder::Asc.to_string(), "ASC");
        assert_eq!(SortOrder::Desc.to_string(), "DESC");
    }
}
