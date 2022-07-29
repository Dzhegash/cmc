use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CmcCategory {
    pub status: Status,
    pub data: Category,
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
    pub last_updated: String,
    pub avg_price_change: f64,
    pub market_cap: f64,
    pub market_cap_change: f64,
    pub volume: f64,
    pub volume_change: f64,
    pub coins: Vec<Coin>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coin {
    pub id: i64,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    pub num_market_pairs: i64,
    pub date_added: String,
    pub tags: Vec<String>,
    pub max_supply: Option<i64>,
    pub circulating_supply: f64,
    pub total_supply: f64,
    pub is_active: i64,
    pub platform: Option<Platform>,
    pub cmc_rank: i64,
    pub is_fiat: i64,
    pub self_reported_circulating_supply: Value,
    pub self_reported_market_cap: Value,
    pub tvl_ratio: Option<f64>,
    pub last_updated: String,
    pub quote: HashMap<String, Currency>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Platform {
    pub id: i64,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    pub token_address: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub tvl: Value,
    pub last_updated: String,
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(
            f,
            "Id: {}\nName: {}\nNum tokens: {}\nAvg Price Change: {}\nMarket Cap: {}\nVolume: {}",
            self.id,
            self.name,
            self.num_tokens,
            self.avg_price_change,
            self.market_cap,
            self.volume
        );

        let _ = writeln!(f, "Coins:");

        for coin in &self.coins {
            let _ = writeln!(f, "{}", coin.name);
        }
        Ok(())
    }
}
