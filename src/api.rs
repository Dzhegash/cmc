use crate::api::cryptocurrency::categories::CmcCategories;
use crate::api::cryptocurrency::coinmarketcap_id_map::CmcIdMap;
use crate::api::cryptocurrency::quotes_latest_v2::{QLv2Id, QLv2Slug, QLv2Symbol};
use crate::api::fiat::coinmarketcap_id_map::CmcIdMapFiat;
use crate::api::key::key_info::{CmcKeyInfo, KeyInfo};
use crate::api::tools::price_conversion_v2::{PCv2Id, PCv2Symbol};
use crate::errors::{ApiError, CmcErrors};
use reqwest::blocking::{Client, RequestBuilder};
use reqwest::StatusCode;

mod cryptocurrency;
mod fiat;
mod key;
mod tests;
mod tools;

const CMC_API_URL: &str = "https://pro-api.coinmarketcap.com/";
type CmcResult<T> = Result<T, CmcErrors>;

#[derive(Clone, Debug)]
pub enum Pass {
    Id,
    Slug,
    Symbol,
}

#[derive(Clone, Debug)]
pub enum Sort {
    Id,
    CmcRank,
}

#[derive(Clone, Debug)]
pub enum SortFiat {
    Id,
    Name,
}

#[derive(Clone, Debug)]
struct Config {
    pass: Pass,
    currency: String,
    currency_id: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            pass: Pass::Symbol,
            currency: "USD".into(),
            currency_id: None,
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

    /// # Set pass:
    ///
    /// - **Id**: Cryptocurrency coinmarketcap id. Example: "1027"
    ///
    /// - **Slug**: Alternatively pass one cryptocurrency slug. Example: "ethereum"
    ///
    /// - **Symbol**: Alternatively pass one cryptocurrency symbol. Example: "BTC"
    ///
    /// **NOTE**: `CoinMarketCap recommend utilizing CMC ID instead of cryptocurrency symbols to securely identify cryptocurrencies with other endpoints and in your own application logic`
    /// (Can be obtained using the method [id_map][id]).
    /// # Example:
    /// ```rust
    /// use cmc::{CmcBuilder, Pass};
    ///
    /// let cmc = CmcBuilder::new("<API KEY>").pass(Pass::Id).build();
    ///
    /// match cmc.price("1027") { // 1027 is Ethereum id.
    ///     Ok(price) => println!("Price: {}", price),
    ///     Err(err) => println!("Error: {}", err),
    /// }
    /// ```
    /// [id]: ./struct.Cmc.html#method.id_map
    pub fn pass(mut self, pass: Pass) -> CmcBuilder {
        self.config.pass = pass;
        self
    }

    /// Optionally calculate market quotes in up to 120 currencies by passing cryptocurrency or fiat.
    /// # Example:
    /// ```rust
    /// use cmc::CmcBuilder;
    ///
    /// let cmc = CmcBuilder::new("<API KEY>").convert("EUR").build();
    ///
    /// match cmc.price("ETH") {
    ///     Ok(price) => println!("Price: {}", price), // In Euro
    ///     Err(err) => println!("Error: {}", err),
    /// }
    /// ```
    pub fn convert<T: Into<String>>(mut self, currency: T) -> CmcBuilder {
        self.config.currency = currency.into().to_uppercase();
        self
    }

    pub fn convert_id<T: Into<String>>(mut self, currency_id: T) -> CmcBuilder {
        self.config.currency_id = Some(currency_id.into());
        self
    }

    /// Returns a Cmc client that uses this CmcBuilder configuration.
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
    /// Constructs a new CoinMarketCap Client.
    pub fn new<T: Into<String>>(api_key: T) -> Self {
        CmcBuilder::new(api_key).build()
    }

    fn add_endpoint(&self, endpoint: &str) -> RequestBuilder {
        self.client
            .get(format!("{}{}", CMC_API_URL, endpoint))
            .header("X-CMC_PRO_API_KEY", &self.api_key)
            .header("Accepts", "application/json")
    }

    /// Returns a mapping of all cryptocurrencies to unique CoinMarketCap ids.
    ///
    /// # Examples
    ///
    /// Parameters:
    /// - `start` Offset the start.
    /// - `limit` Specify the number of results to return.
    /// - `sort` What field to sort the list of cryptocurrencies by.
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use cmc::{Cmc, Sort};
    ///
    /// let cmc = Cmc::new("<API KEY>");
    ///
    /// match cmc.id_map(1, 50, Sort::CmcRank) {
    ///     Ok(map) => println!("{}", map),
    ///     Err(err) => println!("{}", err),
    /// }
    /// ```
    pub fn id_map(&self, start: usize, limit: usize, sort: Sort) -> CmcResult<CmcIdMap> {
        let rb = self
            .add_endpoint("v1/cryptocurrency/map")
            .query(&[("start", start), ("limit", limit)]);

        let resp = match sort {
            Sort::Id => rb.query(&[("sort", "id")]).send()?,
            Sort::CmcRank => rb.query(&[("sort", "cmc_rank")]).send()?,
        };

        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<CmcIdMap>()?;
                Ok(root)
            }
            code => {
                let root = resp.json::<ApiError>()?;
                Err(CmcErrors::ApiError(format!(
                    "Status Code: {}. Error message: {}",
                    code, root.status.error_message
                )))
            }
        }
    }

    /// Returns a mapping of all supported fiat currencies to unique CoinMarketCap ids.
    ///
    /// # Examples
    ///
    /// Parameters:
    /// - `start` Offset the start.
    /// - `limit` Specify the number of results to return.
    /// - `sort` What field to sort the list of currencies by.
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use cmc::{Cmc, SortFiat};
    ///
    /// let cmc = Cmc::new("<API KEY>");
    ///
    /// match cmc.id_map_fiat(1, 100, SortFiat::Name) {
    ///     Ok(map) => println!("{}", map),
    ///     Err(err) => println!("{}", err),
    /// }
    /// ```
    pub fn id_map_fiat(
        &self,
        start: usize,
        limit: usize,
        sort: SortFiat,
    ) -> CmcResult<CmcIdMapFiat> {
        let rb = self
            .add_endpoint("v1/fiat/map")
            .query(&[("start", start), ("limit", limit)]);

        let resp = match sort {
            SortFiat::Id => rb.query(&[("sort", "id")]).send()?,
            SortFiat::Name => rb.query(&[("sort", "name")]).send()?,
        };

        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<CmcIdMapFiat>()?;
                Ok(root)
            }
            code => {
                let root = resp.json::<ApiError>()?;
                Err(CmcErrors::ApiError(format!(
                    "Status Code: {}. Error message: {}",
                    code, root.status.error_message
                )))
            }
        }
    }

    /// Latest price for cryptocurrency in USD.
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use cmc::Cmc;
    ///
    /// let cmc = Cmc::new("<API KEY>");
    ///
    /// match cmc.price("BTC") {
    ///     Ok(price) => println!("Price: {}", price),
    ///     Err(err) => println!("Error: {}", err),
    /// }
    /// ```
    pub fn price<T: Into<String>>(&self, query: T) -> CmcResult<f64> {
        let query = query.into();
        if query.contains(',') {
            return Err(CmcErrors::IncorrectQuery);
        }

        let currency = if let Some(currency_id) = &self.config.currency_id {
            currency_id
        } else {
            &self.config.currency
        };

        match self.config.pass {
            Pass::Symbol => Ok(self.price_by_symbol(&query, currency)?),
            Pass::Id => Ok(self.price_by_id(&query, currency)?),
            Pass::Slug => Ok(self.price_by_slug(&query, currency)?),
        }
    }

    fn price_by_id(&self, id: &str, currency: &str) -> CmcResult<f64> {
        let rb = self
            .add_endpoint("v2/cryptocurrency/quotes/latest")
            .query(&[("id", id)]);

        let resp = if self.config.currency_id.is_some() {
            rb.query(&[("convert_id", currency)]).send()?
        } else {
            rb.query(&[("convert", currency)]).send()?
        };

        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<QLv2Id>()?;
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
                let root = resp.json::<ApiError>()?;
                Err(CmcErrors::ApiError(format!(
                    "Status Code: {}. Error message: {}",
                    code, root.status.error_message
                )))
            }
        }
    }

    fn price_by_slug(&self, slug: &str, currency: &str) -> CmcResult<f64> {
        let rb = self
            .add_endpoint("v2/cryptocurrency/quotes/latest")
            .query(&[("slug", slug.to_lowercase())]);
        let resp = if self.config.currency_id.is_some() {
            rb.query(&[("convert_id", currency)]).send()?
        } else {
            rb.query(&[("convert", currency)]).send()?
        };

        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<QLv2Slug>()?;
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
                let root = resp.json::<ApiError>()?;
                Err(CmcErrors::ApiError(format!(
                    "Status Code: {}. Error message: {}",
                    code, root.status.error_message
                )))
            }
        }
    }

    fn price_by_symbol(&self, symbol: &str, currency: &str) -> CmcResult<f64> {
        let rb = self
            .add_endpoint("v2/cryptocurrency/quotes/latest")
            .query(&[("symbol", symbol)]);

        let resp = if self.config.currency_id.is_some() {
            rb.query(&[("convert_id", currency)]).send()?
        } else {
            rb.query(&[("convert", currency)]).send()?
        };
        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<QLv2Symbol>()?;
                let price = root.data.get(&symbol.to_uppercase()).unwrap()[0]
                    .quote
                    .get(currency)
                    .unwrap()
                    .price;
                Ok(price)
            }
            code => {
                let root = resp.json::<ApiError>()?;
                Err(CmcErrors::ApiError(format!(
                    "Status Code: {}. Error message: {}",
                    code, root.status.error_message
                )))
            }
        }
    }

    /// Returns API key details and usage stats.
    pub fn key_info(&self) -> CmcResult<KeyInfo> {
        let resp = self.add_endpoint("v1/key/info").send()?;
        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<CmcKeyInfo>()?;
                Ok(root.data)
            }
            code => {
                let root = resp.json::<ApiError>()?;
                Err(CmcErrors::ApiError(format!(
                    "Status Code: {}. Error message: {}",
                    code, root.status.error_message
                )))
            }
        }
    }

    /// Convert an amount of one cryptocurrency or fiat currency into one or more different currencies
    /// utilizing the latest market rate for each currency.
    ///
    /// # Examples
    ///
    /// Parameters:
    /// - `amount` An amount of currency to convert.
    /// - `symbol` Alternatively the currency symbol of the base cryptocurrency or fiat to convert from.
    /// - `time` Optional timestamp (Unix or ISO 8601) to reference historical pricing during conversion. If not passed, the current time will be used.
    /// - `convert` Pass  fiat or cryptocurrency symbols to convert the source amount to.
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use cmc::Cmc;
    ///
    /// let cmc = Cmc::new("<API KEY>");
    ///
    /// // 2.5 BTC in EUR
    /// match cmc.price_conversion(2.5, "BTC", None, "EUR") {
    ///     Ok(price) => println!("Total price: {}", price),
    ///     Err(err) => println!("Error: {}", err),
    /// }
    /// ```
    pub fn price_conversion(
        &self,
        amount: f64,
        symbol: &str,
        time: Option<&str>,
        convert: &str,
    ) -> CmcResult<f64> {
        let rb = self
            .add_endpoint("v2/tools/price-conversion")
            .query(&[("amount", amount)])
            .query(&[("symbol", symbol), ("convert", convert)]);

        let resp = match time {
            Some(t) => rb.query(&[("time", t)]).send()?,
            None => rb.send()?,
        };

        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<PCv2Symbol>()?;
                let price = root.data[0]
                    .quote
                    .get(&convert.to_uppercase())
                    .unwrap()
                    .price;
                if let Some(price) = price {
                    Ok(price)
                } else {
                    Err(CmcErrors::NullAnswer)
                }
            }
            code => {
                let root = resp.json::<ApiError>()?;
                Err(CmcErrors::ApiError(format!(
                    "Status Code: {}. Error message: {}",
                    code, root.status.error_message
                )))
            }
        }
    }

    /// Convert an amount of one cryptocurrency or fiat currency into one or more different currencies
    /// utilizing the latest market rate for each currency.
    ///
    /// # Examples
    ///
    /// Parameters:
    /// - `amount` An amount of currency to convert.
    /// - `id` The CoinMarketCap currency ID of the base cryptocurrency or fiat to convert from.
    /// - `time` Optional timestamp (Unix or ISO 8601) to reference historical pricing during conversion. If not passed, the current time will be used.
    /// - `convert_id` Optionally calculate market quotes by CoinMarketCap ID instead of symbol. This option is identical to convert outside of ID format.
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use cmc::Cmc;
    ///
    /// let cmc = Cmc::new("<API KEY>");
    ///
    /// // 3.2 BTC in USD.
    /// match cmc.price_conversion_id(3.2, "1", None, "2781") {
    ///     Ok(price) => println!("Total price: {}", price),
    ///     Err(err) => println!("Error: {}", err),
    /// }
    /// ```
    pub fn price_conversion_id(
        &self,
        amount: f64,
        id: &str,
        time: Option<&str>,
        convert_id: &str,
    ) -> CmcResult<f64> {
        let rb = self
            .add_endpoint("v2/tools/price-conversion")
            .query(&[("amount", amount)])
            .query(&[("id", id), ("convert_id", convert_id)]);

        let resp = match time {
            Some(t) => rb.query(&[("time", t)]).send()?,
            None => rb.send()?,
        };

        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<PCv2Id>()?;
                let price = root
                    .data
                    .quote
                    .get(&convert_id.to_uppercase())
                    .unwrap()
                    .price;
                if let Some(price) = price {
                    Ok(price)
                } else {
                    Err(CmcErrors::NullAnswer)
                }
            }
            code => {
                let root = resp.json::<ApiError>()?;
                Err(CmcErrors::ApiError(format!(
                    "Status Code: {}. Error message: {}",
                    code, root.status.error_message
                )))
            }
        }
    }

    /// Returns information about all coin categories available on CoinMarketCap.
    ///
    /// # Examples
    ///
    /// Parameters:
    /// - `start` Optionally offset the start (1-based index) of the paginated list of items to return.
    /// - `limit` Optionally specify the number of results to return. Use this parameter and the "start" parameter to determine your own pagination size.
    /// - `pass` Cryptocurrency pass (id, slug, symbol)
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use cmc::{CmcBuilder, Pass};
    ///
    /// let cmc = CmcBuilder::new("<API KEY>")
    ///     .pass(Pass::Id)
    ///     .build();
    ///
    /// match cmc.categories(1, 10, "1027") {
    ///     Ok(categories) => println!("{}", categories),
    ///     Err(err) => println!("{}", err),
    /// }
    ///
    /// ```
    pub fn categories<T: Into<String>>(
        &self,
        start: usize,
        limit: usize,
        pass: T,
    ) -> CmcResult<CmcCategories> {
        let query = pass.into();
        let rb = self
            .add_endpoint("v1/cryptocurrency/categories")
            .query(&[("start", start), ("limit", limit)]);

        let resp = match self.config.pass {
            Pass::Symbol => rb.query(&[("symbol", query)]).send()?,
            Pass::Id => rb.query(&[("id", query)]).send()?,
            Pass::Slug => rb.query(&[("slug", query)]).send()?,
        };

        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<CmcCategories>()?;
                Ok(root)
            }
            code => {
                let root = resp.json::<ApiError>()?;
                Err(CmcErrors::ApiError(format!(
                    "Status Code: {}. Error message: {}",
                    code, root.status.error_message
                )))
            }
        }
    }

    // pub fn category()
}
