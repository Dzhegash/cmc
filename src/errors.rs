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
