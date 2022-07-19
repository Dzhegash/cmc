use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CmcKeyInfo {
    pub status: Status,
    pub data: KeyInfo,
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
pub struct KeyInfo {
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

impl Display for KeyInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Plan credit limits:\nDaily: {}\nDaily reset: {}\nMonthly: {}\nMonthly reset: {}\n\
            Rate limit minute: {}\n----------------------\nCredits left:\nDay: {}\nMonth: {}",
            self.plan.credit_limit_daily,
            self.plan.credit_limit_daily_reset,
            self.plan.credit_limit_monthly,
            self.plan.credit_limit_monthly_reset,
            self.plan.rate_limit_minute,
            self.usage.current_day.credits_left,
            self.usage.current_month.credits_left
        )
    }
}
