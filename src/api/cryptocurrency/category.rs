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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub title: String,
    pub description: String,
    pub num_tokens: i64,
    pub last_updated: String,
    pub avg_price_change: f64,
    pub market_cap: f64,
    pub market_cap_change: f64,
    pub volume: f64,
    pub volume_change: f64,
    pub coins: Vec<Coin>,
}