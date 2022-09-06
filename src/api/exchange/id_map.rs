use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CmcExchangeIdMap {
    pub status: Status,
    pub data: Vec<Exchange>,
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
pub struct Exchange {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub is_active: i64,
    pub first_historical_data: String,
    pub last_historical_data: String,
}

impl Display for CmcExchangeIdMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for e in &self.data {
            let _ = writeln!(
                f,
                "Id: {}\nName: {}\nSlug: {}\nIs Active: {}\n---------------",
                e.id, e.name, e.slug, e.is_active
            );
        }
        Ok(())
    }
}
