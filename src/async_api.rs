use crate::api::Config;
use crate::errors::{ApiError, CmcErrors};
use reqwest::StatusCode;
use reqwest::{Client, RequestBuilder};

/// A `CmcBuilder` can be used to create a `Cmc` with custom configuration.
pub struct CmcBuilder {
    api_key: String,
    client: Client,
    config: Config,
}
impl CmcBuilder {
    pub async fn new<T: Into<String>>(api_key: T) -> Self {
        let client = Client::builder().pool_idle_timeout(None).build().unwrap();

        Self {
            api_key: api_key.into(),
            client,
            config: Config::default(),
        }
    }
}
