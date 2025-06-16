use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Pagination {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}
impl Pagination {
    pub fn new(offset: Option<i64>, limit: Option<i64>) -> Self {
        Self { offset, limit }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_default() {
        let pagination = Pagination::default();
        assert_eq!(pagination.offset, None);
        assert_eq!(pagination.limit, None);
    }

    #[test]
    fn test_pagination_new() {
        let pagination = Pagination::new(Some(10), Some(20));
        assert_eq!(pagination.offset, Some(10));
        assert_eq!(pagination.limit, Some(20));
    }

    #[test]
    fn test_pagination_with_partial_values() {
        let pagination = Pagination::new(Some(5), None);
        assert_eq!(pagination.offset, Some(5));
        assert_eq!(pagination.limit, None);

        let pagination = Pagination::new(None, Some(15));
        assert_eq!(pagination.offset, None);
        assert_eq!(pagination.limit, Some(15));
    }
}
