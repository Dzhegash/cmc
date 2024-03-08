use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QLv2Slug {
    pub status: Status,
    pub data: HashMap<String, CryptoCurrency>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QLv2Id {
    pub status: Status,
    pub data: HashMap<String, CryptoCurrency>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QLv2Symbol {
    pub status: Status,
    pub data: HashMap<String, Vec<CryptoCurrency>>,
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
pub struct CryptoCurrency {
    pub id: i64,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    pub num_market_pairs: Value,
    pub date_added: Value,
    pub tags: Vec<Tag>,
    pub max_supply: Value,
    pub circulating_supply: Value,
    pub total_supply: Value,
    pub is_active: i64,
    pub platform: Value,
    pub cmc_rank: Value,
    pub is_fiat: i64,
    pub self_reported_circulating_supply: Value,
    pub self_reported_market_cap: Value,
    pub tvl_ratio: Value,
    pub last_updated: String,
    pub quote: HashMap<String, Currency>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub slug: String,
    pub name: String,
    pub category: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Currency {
    pub price: Option<f64>,
    pub volume_24h: Value,
    pub volume_change_24h: Value,
    pub percent_change_1h: Value,
    pub percent_change_24h: Value,
    pub percent_change_7d: Value,
    pub percent_change_30d: Value,
    pub percent_change_60d: Value,
    pub percent_change_90d: Value,
    pub market_cap: Value,
    pub market_cap_dominance: Value,
    pub fully_diluted_market_cap: Value,
    pub tvl: Value,
    pub last_updated: String,
}
