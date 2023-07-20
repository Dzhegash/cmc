use crate::api::cryptocurrency::{CmcIdMap, QLv2Id, QLv2Slug, QLv2Symbol};
use crate::api::fiat::CmcFiatIdMap;
use crate::api::Config;
use crate::errors::{ApiError, CmcErrors};
use crate::{Pass, Sort, SortFiat};
use reqwest::StatusCode;
use reqwest::{Client, RequestBuilder};

const CMC_API_URL: &str = "https://pro-api.coinmarketcap.com/";
type CmcResult<T> = Result<T, CmcErrors>;

/// A `CmcBuilder` can be used to create a `Cmc` with custom configuration.
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
    /// (Can be obtained using the method [id_map()][id]).
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

    /// Optionally calculate market quotes in up to 120 currencies by passing cryptocurrency or fiat.
    /// # Example:
    /// ```rust
    /// use cmc::CmcBuilder;
    ///
    /// let cmc = CmcBuilder::new("<API KEY>").convert_id("1027").build();
    ///
    /// match cmc.price("BTC") {
    ///     Ok(price) => println!("Price: {}", price), // In ETH
    ///     Err(err) => println!("Error: {}", err),
    /// }
    /// ```
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

/// A `Cmc` can be used to create a CoinMarketCap client with default configuration.
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
    /// # Example:
    ///
    /// Parameters:
    /// - `start` Offset the start.
    /// - `limit` Specify the number of results to return.
    /// - `sort` What field to sort the list of cryptocurrencies by.
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
    #[cfg(feature = "cryptocurrency")]
    pub async fn id_map(&self, start: usize, limit: usize, sort: Sort) -> CmcResult<CmcIdMap> {
        let rb = self
            .add_endpoint("v1/cryptocurrency/map")
            .query(&[("start", start), ("limit", limit)]);

        let resp = match sort {
            Sort::Id => rb.query(&[("sort", "id")]).send().await?,
            Sort::CmcRank => rb.query(&[("sort", "cmc_rank")]).send().await?,
        };

        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<CmcIdMap>().await?;
                Ok(root)
            }
            code => {
                let root = resp.json::<ApiError>().await?;
                Err(CmcErrors::ApiError(format!(
                    "Status Code: {}. Error message: {}",
                    code, root.status.error_message
                )))
            }
        }
    }

    /// Returns a mapping of all supported fiat currencies to unique CoinMarketCap ids.
    ///
    /// # Example:
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
    /// match cmc.fiat_id_map(1, 100, SortFiat::Name) {
    ///     Ok(map) => println!("{}", map),
    ///     Err(err) => println!("{}", err),
    /// }
    /// ```
    #[cfg(feature = "fiat")]
    pub async fn fiat_id_map(
        &self,
        start: usize,
        limit: usize,
        sort: SortFiat,
    ) -> CmcResult<CmcFiatIdMap> {
        let rb = self
            .add_endpoint("v1/fiat/map")
            .query(&[("start", start), ("limit", limit)]);

        let resp = match sort {
            SortFiat::Id => rb.query(&[("sort", "id")]).send().await?,
            SortFiat::Name => rb.query(&[("sort", "name")]).send().await?,
        };

        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<CmcFiatIdMap>().await?;
                Ok(root)
            }
            code => {
                let root = resp.json::<ApiError>().await?;
                Err(CmcErrors::ApiError(format!(
                    "Status Code: {}. Error message: {}",
                    code, root.status.error_message
                )))
            }
        }
    }

    /// Latest price for cryptocurrency in USD.
    ///
    /// # Example:
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
    #[cfg(feature = "cryptocurrency")]
    pub async fn price<T: Into<String>>(&self, query: T) -> CmcResult<f64> {
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
            Pass::Symbol => Ok(self.price_by_symbol(&query, currency).await?),
            Pass::Id => Ok(self.price_by_id(&query, currency).await?),
            Pass::Slug => Ok(self.price_by_slug(&query, currency).await?),
            Pass::Address => Err(CmcErrors::PassIncompatible),
        }
    }

    #[cfg(feature = "cryptocurrency")]
    async fn price_by_id(&self, id: &str, currency: &str) -> CmcResult<f64> {
        let rb = self
            .add_endpoint("v2/cryptocurrency/quotes/latest")
            .query(&[("id", id)]);

        let resp = if self.config.currency_id.is_some() {
            rb.query(&[("convert_id", currency)]).send().await?
        } else {
            rb.query(&[("convert", currency)]).send().await?
        };

        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<QLv2Id>().await?;
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
                let root = resp.json::<ApiError>().await?;
                Err(CmcErrors::ApiError(format!(
                    "Status Code: {}. Error message: {}",
                    code, root.status.error_message
                )))
            }
        }
    }

    #[cfg(feature = "cryptocurrency")]
    async fn price_by_slug(&self, slug: &str, currency: &str) -> CmcResult<f64> {
        let rb = self
            .add_endpoint("v2/cryptocurrency/quotes/latest")
            .query(&[("slug", slug.to_lowercase())]);
        let resp = if self.config.currency_id.is_some() {
            rb.query(&[("convert_id", currency)]).send().await?
        } else {
            rb.query(&[("convert", currency)]).send().await?
        };

        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<QLv2Slug>().await?;
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
                let root = resp.json::<ApiError>().await?;
                Err(CmcErrors::ApiError(format!(
                    "Status Code: {}. Error message: {}",
                    code, root.status.error_message
                )))
            }
        }
    }

    #[cfg(feature = "cryptocurrency")]
    async fn price_by_symbol(&self, symbol: &str, currency: &str) -> CmcResult<f64> {
        let rb = self
            .add_endpoint("v2/cryptocurrency/quotes/latest")
            .query(&[("symbol", symbol)]);

        let resp = if self.config.currency_id.is_some() {
            rb.query(&[("convert_id", currency)]).send().await?
        } else {
            rb.query(&[("convert", currency)]).send().await?
        };
        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<QLv2Symbol>().await?;
                let price = root.data.get(&symbol.to_uppercase()).unwrap()[0]
                    .quote
                    .get(currency)
                    .unwrap()
                    .price;
                Ok(price)
            }
            code => {
                let root = resp.json::<ApiError>().await?;
                Err(CmcErrors::ApiError(format!(
                    "Status Code: {}. Error message: {}",
                    code, root.status.error_message
                )))
            }
        }
    }
}
