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
