use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyInfo {
    pub status: Status,
    pub data: Data,
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
pub struct Data {
    pub plan: Plan,
    pub usage: Usage,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    pub credit_limit_daily: i64,
    pub credit_limit_daily_reset: String,
    pub credit_limit_daily_reset_timestamp: String,
    pub credit_limit_monthly: i64,
    pub credit_limit_monthly_reset: String,
    pub credit_limit_monthly_reset_timestamp: String,
    pub rate_limit_minute: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    pub current_minute: CurrentMinute,
    pub current_day: CurrentDay,
    pub current_month: CurrentMonth,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentMinute {
    pub requests_made: i64,
    pub requests_left: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentDay {
    pub credits_used: i64,
    pub credits_left: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentMonth {
    pub credits_used: i64,
    pub credits_left: i64,
}