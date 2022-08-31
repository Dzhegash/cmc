use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CmcExchangeIdMap {
    pub status: Status,
    pub data: Vec<Exchange>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub timestamp: String,
    pub error_code: i64,
    pub error_message: Value,
    pub elapsed: i64,
    pub credit_count: i64,
    pub notice: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Exchange {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub is_active: i64,
    pub first_historical_data: String,
    pub last_historical_data: String,
}