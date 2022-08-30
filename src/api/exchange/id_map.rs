use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Exchange {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub is_active: i64,
    pub first_historical_data: String,
    pub last_historical_data: String,
}