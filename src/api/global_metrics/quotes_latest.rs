use serde::{Deserialize, Serialize};
use serde_json::Value;

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
