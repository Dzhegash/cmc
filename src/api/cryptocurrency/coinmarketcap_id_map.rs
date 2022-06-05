use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoinMarketCapIdMap {
    pub status: Status,
    pub data: Vec<Cryptocurrency>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub timestamp: String,
    pub error_code: i64,
    pub error_message: Value,
    pub elapsed: i64,
    pub credit_count: i64,
    pub notice: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cryptocurrency {
    pub id: i64,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    pub rank: i64,
    pub is_active: i64,
    pub first_historical_data: String,
    pub last_historical_data: String,
    pub platform: Value,
}
