use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrudFilter {
    pub field: String,
    pub op: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrudQuery {
    pub filters: Vec<CrudFilter>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl Default for CrudQuery {
    fn default() -> Self {
        Self {
            filters: Vec::new(),
            limit: None,
            offset: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrudResult<T> {
    pub data: Option<T>,
    pub list: Vec<T>,
    pub count: usize,
    pub success: bool,
    pub message: Option<String>,
}

impl<T> CrudResult<T> {
    pub fn success_data(data: T) -> Self {
        Self {
            data: Some(data),
            list: Vec::new(),
            count: 1,
            success: true,
            message: None,
        }
    }

    pub fn success_list(list: Vec<T>, count: usize) -> Self {
        Self {
            data: None,
            list,
            count,
            success: true,
            message: None,
        }
    }

    pub fn success_count(count: usize) -> Self {
        Self {
            data: None,
            list: Vec::new(),
            count,
            success: true,
            message: None,
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            data: None,
            list: Vec::new(),
            count: 0,
            success: false,
            message: Some(message.to_string()),
        }
    }
}
