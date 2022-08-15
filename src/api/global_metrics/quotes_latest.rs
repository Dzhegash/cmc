use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CmcGlobalMetrics {
    pub status: Status,
    pub data: GlobalMetrics,
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
pub struct GlobalMetrics {
    pub active_cryptocurrencies: i64,
    pub total_cryptocurrencies: i64,
    pub active_market_pairs: i64,
    pub active_exchanges: i64,
    pub total_exchanges: i64,
    pub eth_dominance: f64,
    pub btc_dominance: f64,
    pub eth_dominance_yesterday: f64,
    pub btc_dominance_yesterday: f64,
    pub eth_dominance_24h_percentage_change: f64,
    pub btc_dominance_24h_percentage_change: f64,
    pub defi_volume_24h: f64,
    pub defi_volume_24h_reported: f64,
    pub defi_market_cap: f64,
    pub defi_24h_percentage_change: f64,
    pub stablecoin_volume_24h: f64,
    pub stablecoin_volume_24h_reported: f64,
    pub stablecoin_market_cap: f64,
    pub stablecoin_24h_percentage_change: f64,
    pub derivatives_volume_24h: f64,
    pub derivatives_volume_24h_reported: f64,
    pub derivatives_24h_percentage_change: f64,
    pub quote: HashMap<String, Currency>,
    pub last_updated: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Currency {
    pub total_market_cap: f64,
    pub total_volume_24h: f64,
    pub total_volume_24h_reported: f64,
    pub altcoin_volume_24h: f64,
    pub altcoin_volume_24h_reported: f64,
    pub altcoin_market_cap: f64,
    pub defi_volume_24h: f64,
    pub defi_volume_24h_reported: f64,
    pub defi_24h_percentage_change: f64,
    pub defi_market_cap: f64,
    pub stablecoin_volume_24h: f64,
    pub stablecoin_volume_24h_reported: f64,
    pub stablecoin_24h_percentage_change: f64,
    pub stablecoin_market_cap: f64,
    pub derivatives_volume_24h: f64,
    pub derivatives_volume_24h_reported: f64,
    pub derivatives_24h_percentage_change: f64,
    pub last_updated: String,
    pub total_market_cap_yesterday: f64,
    pub total_volume_24h_yesterday: f64,
    pub total_market_cap_yesterday_percentage_change: f64,
    pub total_volume_24h_yesterday_percentage_change: f64,
}
