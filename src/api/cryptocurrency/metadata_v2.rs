use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MDv2 {
    pub status: Status,
    pub data: HashMap<String, Metadata>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MDv2Symbol {
    pub status: Status,
    pub data: HashMap<String, Vec<Metadata>>,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Urls {
    pub website: Vec<Value>,
    pub twitter: Vec<Value>,
    pub message_board: Vec<Value>,
    pub chat: Vec<Value>,
    pub facebook: Vec<Value>,
    pub explorer: Vec<Value>,
    pub reddit: Vec<Value>,
    pub technical_doc: Vec<Value>,
    pub source_code: Vec<Value>,
    pub announcement: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Platform {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub symbol: String,
    pub token_address: String,
}
