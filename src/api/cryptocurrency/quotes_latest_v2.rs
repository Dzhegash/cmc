use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuotesLatestV2SlugOrId {
    pub status: Status,
    pub data: HashMap<String, CryptoCurrency>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuotesLatestV2Symbol {
    pub status: Status,
    pub data: HashMap<String, Vec<CryptoCurrency>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuotesLatestV2error {
    pub status: StatusError,
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
pub struct StatusError {
    pub timestamp: String,
    pub error_code: i64,
    pub error_message: Value,
    pub elapsed: i64,
    pub credit_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CryptoCurrency {
    pub id: i64,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    pub num_market_pairs: Value,
    pub date_added: String,
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
    pub last_updated: String,
    pub quote: HashMap<String, Currency>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub slug: String,
    pub name: String,
    pub category: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Currency {
    pub price: f64,
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
    pub last_updated: String,
}
