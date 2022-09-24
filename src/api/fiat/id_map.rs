use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CmcFiatIdMap {
    pub status: Status,
    pub data: Vec<Currency>,
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
pub struct Currency {
    pub id: i64,
    pub name: String,
    pub sign: String,
    pub symbol: String,
}

impl Display for CmcFiatIdMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for fc in &self.data {
            let _ = writeln!(
                f,
                "Id: {}\nName: {}\nSign: {}\nSymbol: {}\n---------------",
                fc.id, fc.name, fc.sign, fc.symbol
            );
        }
        Ok(())
    }
}
