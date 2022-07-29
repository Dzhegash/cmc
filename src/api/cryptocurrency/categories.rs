use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CmcCategories {
    pub status: Status,
    pub data: Vec<Category>,
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
pub struct Category {
    pub id: String,
    pub name: String,
    pub title: String,
    pub description: String,
    pub num_tokens: i64,
    pub avg_price_change: f64,
    pub market_cap: f64,
    pub market_cap_change: f64,
    pub volume: f64,
    pub volume_change: f64,
    pub last_updated: String,
}

impl Display for CmcCategories {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for cc in &self.data {
            let _ = writeln!(
                f,
                "Id: {}\nName: {}\nAvg Price Change: {}\nMarket Cap: {}\nVolume: {}\n-----------------------------",
                cc.id ,cc.name, cc.avg_price_change, cc.market_cap, cc.volume
            );
        }
        Ok(())
    }
}
