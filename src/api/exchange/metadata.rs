use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Exchange {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Value,
    pub notice: String,
    pub logo: String,
    pub countries: Vec<Value>,
    pub fiats: Vec<String>,
    pub urls: Urls,
    pub tags: Value,
    #[serde(rename = "type")]
    pub type_field: String,
    pub date_launched: Value,
    pub is_hidden: i64,
    pub is_redistributable: Value,
    pub maker_fee: f64,
    pub taker_fee: f64,
    pub spot_volume_usd: Value,
    pub spot_volume_last_updated: Value,
    pub weekly_visits: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Urls {
    pub chat: Vec<Value>,
    pub twitter: Vec<Value>,
    pub blog: Vec<Value>,
    pub fee: Vec<Value>,
    pub website: Vec<Value>,
}