use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub timestamp: String,
    pub error_code: i64,
    pub error_message: Value,
    pub elapsed: i64,
    pub credit_count: i64,
    pub notice: Value,
}