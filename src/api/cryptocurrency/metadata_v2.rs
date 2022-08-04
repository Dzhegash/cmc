use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CmcMetadata {
    pub status: Status,
    pub data: HashMap<String, Metadata>,
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
pub struct Metadata {
    pub id: i64,
    pub name: String,
    pub symbol: String,
    pub category: String,
    pub description: String,
    pub slug: String,
    pub logo: String,
    pub subreddit: String,
    pub notice: String,
    pub tags: Vec<String>,
    #[serde(rename = "tag-names")]
    pub tag_names: Vec<String>,
    #[serde(rename = "tag-groups")]
    pub tag_groups: Vec<String>,
    pub urls: Urls,
    pub platform: Option<Platform>,
    pub date_added: String,
    pub twitter_username: String,
    pub is_hidden: i64,
    pub date_launched: Value,
    pub contract_address: Vec<Value>,
    pub self_reported_circulating_supply: Value,
    pub self_reported_tags: Value,
    pub self_reported_market_cap: Value,
}
