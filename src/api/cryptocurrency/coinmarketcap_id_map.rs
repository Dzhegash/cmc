use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CmcIdMap {
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

impl CmcIdMap {
    pub fn display(&self) -> String {
        let mut s = String::new();
        for cc in &self.data {
            let _ = writeln!(
                s,
                "CMC Id: {}\nName: {}\nSymbol: {}\nSlug: {}\nRank: {}\n---------------",
                cc.id, cc.name, cc.symbol, cc.slug, cc.rank
            );
        }
        s
    }
}
