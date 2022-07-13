use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CmcErrors {
    #[error("Request Error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Query must not contain commas")]
    IncorrectQuery,
    #[error("Api answer is null")]
    NullAnswer,
    #[error("API Error: {0}")]
    ApiError(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiError {
    pub status: StatusError,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusError {
    pub timestamp: String,
    pub error_code: i64,
    pub error_message: Value,
    pub elapsed: i64,
    pub credit_count: i64,
}
