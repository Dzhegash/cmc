use crate::api::cryptocurrency::quotes_latest_v2::*;
use crate::errors::CmcErrors;
use reqwest::blocking::{Client, RequestBuilder};
use reqwest::StatusCode;

mod cryptocurrency;
mod tests;

const CMC_API_URL: &str = "https://pro-api.coinmarketcap.com/";
type CmcResult<T> = Result<T, CmcErrors>;

#[derive(Clone, Debug)]
pub enum Pass {
    Id,
    Slug,
    Symbol,
}

#[derive(Clone, Debug)]
struct Config {
    pass: Pass,
    currency: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            pass: Pass::Symbol,
            currency: "USD".into(),
        }
    }
}

pub struct CmcBuilder {
    api_key: String,
    client: Client,
    config: Config,
}

impl CmcBuilder {
    pub fn new<T: Into<String>>(api_key: T) -> Self {
        let client = Client::builder().pool_idle_timeout(None).build().unwrap();

        Self {
            api_key: api_key.into(),
            client,
            config: Config::default(),
        }
    }

    pub fn pass(mut self, pass: Pass) -> CmcBuilder {
        self.config.pass = pass;
        self
    }

    pub fn build(self) -> Cmc {
        Cmc {
            api_key: self.api_key,
            client: self.client,
            config: self.config,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cmc {
    api_key: String,
    client: Client,
    config: Config,
}

impl Cmc {
    pub fn new<T: Into<String>>(api_key: T) -> Self {
        CmcBuilder::new(api_key).build()
    }

    fn add_endpoint(&self, endpoint: &str) -> RequestBuilder {
        self.client
            .get(format!("{}{}", CMC_API_URL, endpoint))
            .header("X-CMC_PRO_API_KEY", &self.api_key)
            .header("Accepts", "application/json")
    }

    /// Latest price for cryptocurrency (Symbol).
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use cmc::Cmc;
    ///
    /// let cmc = Cmc::new("<your API key>");
    /// match cmc.price("BTC") {
    ///     Ok(price) => println!("{}", price),
    ///     Err(err) => println!("Error: {}", err),
    /// }
    ///
    /// ```
    pub fn price<T: Into<String>>(&self, query: T) -> CmcResult<f64> {
        let query = query.into();
        if query.contains(',') {
            return Err(CmcErrors::IncorrectQuery);
        }
        let currency = &self.config.currency;
        match self.config.pass {
            Pass::Symbol => Ok(Cmc::price_by_symbol(self, &query, currency)?),
            Pass::Id => Ok(Cmc::price_by_id(self, &query, currency)?),
            Pass::Slug => Ok(Cmc::price_by_slug(self, &query, currency)?),
        }
    }

    fn price_by_id(cmc: &Cmc, id: &str, currency: &str) -> CmcResult<f64> {
        let resp = cmc
            .add_endpoint("v2/cryptocurrency/quotes/latest")
            .query(&[("id", id)])
            .send()?;
        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<QuotesLatestV2SlugOrId>()?;
                let price = root
                    .data
                    .get(id)
                    .unwrap()
                    .quote
                    .get(currency)
                    .unwrap()
                    .price;
                Ok(price)
            }
            code => {
                let root = resp.json::<QuotesLatestV2error>()?;
                Err(CmcErrors::ApiError(format!(
                    "Status Code: {}. Error message: {}",
                    code, root.status.error_message
                )))
            }
        }
    }

    fn price_by_slug(cmc: &Cmc, slug: &str, currency: &str) -> CmcResult<f64> {
        let resp = cmc
            .add_endpoint("v2/cryptocurrency/quotes/latest")
            .query(&[("slug", slug.to_lowercase())])
            .send()?;
        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<QuotesLatestV2SlugOrId>()?;
                let slug_id = root.data.iter().next().unwrap().0;
                let price = root
                    .data
                    .get(slug_id)
                    .unwrap()
                    .quote
                    .get(currency)
                    .unwrap()
                    .price;
                Ok(price)
            }
            code => {
                let root = resp.json::<QuotesLatestV2error>()?;
                Err(CmcErrors::ApiError(format!(
                    "Status Code: {}. Error message: {}",
                    code, root.status.error_message
                )))
            }
        }
    }

    fn price_by_symbol(cmc: &Cmc, symbol: &str, currency: &str) -> CmcResult<f64> {
        let resp = cmc
            .add_endpoint("v2/cryptocurrency/quotes/latest")
            .query(&[("symbol", symbol)])
            .send()?;
        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<QuotesLatestV2Symbol>()?;
                let price = root.data.get(&symbol.to_uppercase()).unwrap()[0]
                    .quote
                    .get(currency)
                    .unwrap()
                    .price;
                Ok(price)
            }
            code => {
                let root = resp.json::<QuotesLatestV2error>()?;
                Err(CmcErrors::ApiError(format!(
                    "Status Code: {}. Error message: {}",
                    code, root.status.error_message
                )))
            }
        }
    }
}
