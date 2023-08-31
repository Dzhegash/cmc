use crate::api::cryptocurrency::{
    Category, CmcCategories, CmcCategory, CmcIdMap, MDv2, MDv2Symbol, Metadata, QLv2Id, QLv2Slug,
    QLv2Symbol,
};
use crate::api::exchange::{CmcExchangeIdMap, ExchangeMetadata};
use crate::api::fiat::CmcFiatIdMap;
use crate::api::global_metrics::{CmcGlobalMetrics, GlobalMetrics};
use crate::api::key::{CmcKeyInfo, KeyInfo};
use crate::api::tools::{PCv2Id, PCv2Symbol};
use crate::api::{CmcResult, Config, CMC_API_URL};
use crate::errors::{ApiError, CmcErrors};
use crate::{ListingStatusExchange, Pass, Sort, SortExchange, SortFiat};
use reqwest::StatusCode;
use reqwest::{Client, RequestBuilder};

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

    /// Returns the latest market quote for 1 or more cryptocurrencies (using id's).
    #[cfg(feature = "cryptocurrency")]
    pub async fn quotes_latest_by_id<T: Into<String>>(&self, ids: T) -> CmcResult<QLv2Id> {
        let ids = ids.into();

        let rb = self
            .add_endpoint("v2/cryptocurrency/quotes/latest")
            .query(&[("id", ids)]);

        let resp = if let Some(currency_id) = &self.config.currency_id {
            rb.query(&[("convert_id", currency_id)]).send().await?
        } else {
            rb.query(&[("convert", &self.config.currency)])
                .send()
                .await?
        };

        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<QLv2Id>().await?;
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

    /// Returns the latest market quote for 1 or more cryptocurrencies (using slug's).
    #[cfg(feature = "cryptocurrency")]
    pub async fn quotes_latest_by_slug<T: Into<String>>(&self, slugs: T) -> CmcResult<QLv2Slug> {
        let slugs = slugs.into();

        let rb = self
            .add_endpoint("v2/cryptocurrency/quotes/latest")
            .query(&[("slug", slugs)]);

        let resp = if let Some(currency_id) = &self.config.currency_id {
            rb.query(&[("convert_id", currency_id)]).send().await?
        } else {
            rb.query(&[("convert", &self.config.currency)])
                .send()
                .await?
        };

        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<QLv2Slug>().await?;
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

    /// Returns the latest market quote for 1 or more cryptocurrencies (using symbol's).
    #[cfg(feature = "cryptocurrency")]
    pub async fn quotes_latest_by_symbol<T: Into<String>>(
        &self,
        symbols: T,
    ) -> CmcResult<QLv2Symbol> {
        let symbols = symbols.into();

        let rb = self
            .add_endpoint("v2/cryptocurrency/quotes/latest")
            .query(&[("symbol", symbols)]);

        let resp = if let Some(currency_id) = &self.config.currency_id {
            rb.query(&[("convert_id", currency_id)]).send().await?
        } else {
            rb.query(&[("convert", &self.config.currency)])
                .send()
                .await?
        };

        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<QLv2Symbol>().await?;
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

    /// Returns API key details and usage stats.
    #[cfg(feature = "key")]
    pub async fn key_info(&self) -> CmcResult<KeyInfo> {
        let resp = self.add_endpoint("v1/key/info").send().await?;
        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<CmcKeyInfo>().await?;
                Ok(root.data)
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

    /// Convert an amount of one cryptocurrency or fiat currency into one or more different currencies
    /// utilizing the latest market rate for each currency.
    ///
    /// # Example:
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
    #[cfg(feature = "tools")]
    pub async fn price_conversion(
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
            Some(t) => rb.query(&[("time", t)]).send().await?,
            None => rb.send().await?,
        };

        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<PCv2Symbol>().await?;
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
                let root = resp.json::<ApiError>().await?;
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
    /// # Example:
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
    /// // 1.6 ETH in Monero (XMR).
    /// match cmc.price_conversion_id(1.6, "1027", None, "328") {
    ///     Ok(price) => println!("Total price: {}", price),
    ///     Err(err) => println!("Error: {}", err),
    /// }
    /// ```
    #[cfg(feature = "tools")]
    pub async fn price_conversion_id(
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
            Some(t) => rb.query(&[("time", t)]).send().await?,
            None => rb.send().await?,
        };

        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<PCv2Id>().await?;
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
                let root = resp.json::<ApiError>().await?;
                Err(CmcErrors::ApiError(format!(
                    "Status Code: {}. Error message: {}",
                    code, root.status.error_message
                )))
            }
        }
    }

    /// Returns information about all coin categories available on CoinMarketCap.
    ///
    /// # Example:
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
    ///     Ok(categories) => println!("{categories}"),
    ///     Err(err) => println!("{err}"),
    /// }
    /// ```
    #[cfg(feature = "cryptocurrency")]
    pub async fn categories<T: Into<String>>(
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
            Pass::Symbol => rb.query(&[("symbol", query)]).send().await?,
            Pass::Id => rb.query(&[("id", query)]).send().await?,
            Pass::Slug => rb.query(&[("slug", query)]).send().await?,
            Pass::Address => return Err(CmcErrors::PassIncompatible),
        };

        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<CmcCategories>().await?;
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

    /// Returns information about a single coin category available on CoinMarketCap.
    ///
    /// # Example:
    ///
    /// Parameters:
    /// - `id` The Category ID. This can be found using the [categories()].
    /// - `start` Optionally offset the start (1-based index) of the paginated list of coins to return.
    /// - `limit` Optionally specify the number of coins to return. Use this parameter and the "start" parameter to determine your own pagination size.
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use cmc::CmcBuilder;
    ///
    /// let cmc = CmcBuilder::new("<API KEY>")
    ///     .convert("EUR")
    ///     .build();
    ///
    /// match cmc.category("605e2ce9d41eae1066535f7c", 1, 10) {
    ///     Ok(category) => println!("{category}"),
    ///     Err(err) => println!("{err}"),
    /// }
    /// ```
    /// [categories()]: ./struct.Cmc.html#method.categories
    #[cfg(feature = "cryptocurrency")]
    pub async fn category(&self, id: &str, start: usize, limit: usize) -> CmcResult<Category> {
        let rb = self
            .add_endpoint("v1/cryptocurrency/category")
            .query(&[("id", id)])
            .query(&[("start", start), ("limit", limit)]);

        let resp = if let Some(currency_id) = &self.config.currency_id {
            rb.query(&[("convert_id", currency_id)]).send().await?
        } else {
            rb.query(&[("convert", &self.config.currency)])
                .send()
                .await?
        };

        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<CmcCategory>().await?;
                Ok(root.data)
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

    /// Returns all static metadata available for one or more cryptocurrencies.
    /// This information includes details like logo, description, official website URL, social links,
    /// and links to a cryptocurrency's technical documentation.
    ///
    /// Parameters:
    ///
    /// - **Id**: Cryptocurrency coinmarketcap id. Example: "1027"
    ///
    /// - **Slug**: Alternatively pass one cryptocurrency slug. Example: "ethereum"
    ///
    /// - **Symbol**: Alternatively pass one cryptocurrency symbol. Example: "BTC"
    ///
    /// - **Address**: Alternatively pass in a contract address. Example: "0xc40af1e4fecfa05ce6bab79dcd8b373d2e436c4e"
    ///
    /// **NOTE**: `CoinMarketCap recommend utilizing CMC ID instead of cryptocurrency symbols to securely identify cryptocurrencies with other endpoints and in your own application logic`
    /// (Can be obtained using the method [id_map()][id]).
    /// ```rust
    /// use cmc::{CmcBuilder, Pass};
    ///
    /// let cmc = CmcBuilder::new("<API KEY>")
    ///     .pass(Pass::Id)
    ///     .build();
    /// // Cryptocurrency metadata.
    /// match cmc.metadata("1027") {
    ///     Ok(metadata) => println!("{}", metadata.description),
    ///     Err(err) => println!("{}", err),
    /// }
    ///
    /// let cmc = CmcBuilder::new("<API KEY>")
    ///     .pass(Pass::Address)
    ///     .build();
    /// // Contract address metadata.
    /// match cmc.metadata("0xc40af1e4fecfa05ce6bab79dcd8b373d2e436c4e") {
    ///     Ok(metadata) => println!("{}", metadata.description),
    ///     Err(err) => println!("{}", err),
    /// }
    ///```
    /// [id]: ./struct.Cmc.html#method.id_map
    #[cfg(feature = "cryptocurrency")]
    pub async fn metadata<T: Into<String>>(&self, query: T) -> CmcResult<Metadata> {
        let query = query.into();

        if query.contains(',') {
            return Err(CmcErrors::IncorrectQuery);
        }

        let rb = self.add_endpoint("v2/cryptocurrency/info");

        let resp = match self.config.pass {
            Pass::Symbol => rb.query(&[("symbol", &query)]).send().await?,
            Pass::Id => rb.query(&[("id", &query)]).send().await?,
            Pass::Slug => rb.query(&[("slug", &query.to_lowercase())]).send().await?,
            Pass::Address => rb.query(&[("address", &query)]).send().await?,
        };

        match resp.status() {
            StatusCode::OK => match self.config.pass {
                Pass::Symbol => {
                    let mut root = resp.json::<MDv2Symbol>().await?;
                    let md_vec = root.data.remove(&query).unwrap();
                    Ok(md_vec[0].clone())
                }
                Pass::Slug | Pass::Address => {
                    let mut root = resp.json::<MDv2>().await?;
                    let slug_id = root.data.iter().next().unwrap().0.to_owned();
                    let md = root.data.remove(&slug_id).unwrap();
                    Ok(md)
                }
                Pass::Id => {
                    let mut root = resp.json::<MDv2>().await?;
                    let md = root.data.remove(&query).unwrap();
                    Ok(md)
                }
            },
            code => {
                let root = resp.json::<ApiError>().await?;
                Err(CmcErrors::ApiError(format!(
                    "Status Code: {}. Error message: {}",
                    code, root.status.error_message
                )))
            }
        }
    }

    /// Returns the latest global cryptocurrency market metrics. Use the [convert()] to return
    /// market values in multiple fiat and cryptocurrency conversions in the same call.
    ///
    /// ```rust
    /// use cmc::CmcBuilder;
    ///
    /// let cmc = CmcBuilder::new("<API KEY>")
    ///     .convert("EUR")
    ///     .build();
    ///
    /// match cmc.global_metrics() {
    ///     Ok(gm) => println!("{}", gm.btc_dominance),
    ///     Err(err) => println!("{}", err),
    /// }
    /// ```
    /// [convert()]: ./struct.CmcBuilder.html#method.convert
    #[cfg(feature = "global_metrics")]
    pub async fn global_metrics(&self) -> CmcResult<GlobalMetrics> {
        let rb = self.add_endpoint("v1/global-metrics/quotes/latest");

        let resp = if let Some(currency_id) = &self.config.currency_id {
            rb.query(&[("convert_id", currency_id)]).send().await?
        } else {
            rb.query(&[("convert", &self.config.currency)])
                .send()
                .await?
        };

        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<CmcGlobalMetrics>().await?;
                Ok(root.data)
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

    /// Returns all static metadata for one or more exchanges. This information includes details
    /// like launch date, logo, official website URL, social links, and market fee documentation URL.
    ///
    /// # Examples:
    ///
    /// Parameters:
    ///
    /// - **Id**: One or more comma-separated CoinMarketCap cryptocurrency exchange ids. Example: "270,271"
    ///
    /// - **Slug**: Alternatively, one or more comma-separated exchange names in URL friendly
    ///   shorthand "slug" format (all lowercase, spaces replaced with hyphens). Example: "binance,gdax".
    ///
    /// ```rust
    /// use cmc::{CmcBuilder, Pass};
    ///
    /// // using Id
    /// let cmc = CmcBuilder::new("<API KEY>")
    ///     .pass(Pass::Id)
    ///     .build();
    ///
    /// match cmc.exchange_metadata("270") {
    ///     Ok(metadata) => println!("{}", metadata.data.get("270").unwrap().name),
    ///     Err(err) => println!("{}", err),
    /// }
    ///
    /// // using Slug
    /// let cmc = CmcBuilder::new("<API KEY>")
    ///     .pass(Pass::Slug)
    ///     .build();
    ///
    /// match cmc.exchange_metadata("binance") {
    ///     Ok(metadata) => println!("{}", metadata.data.get("binance").unwrap().name),
    ///     Err(err) => println!("{}", err),
    /// }
    /// ```
    #[cfg(feature = "exchange")]
    pub async fn exchange_metadata<T: Into<String>>(
        &self,
        exchange: T,
    ) -> CmcResult<ExchangeMetadata> {
        let exchange = exchange.into();

        let rb = self.add_endpoint("v1/exchange/info");

        let resp = match self.config.pass {
            Pass::Symbol => return Err(CmcErrors::PassIncompatible),
            Pass::Id => rb.query(&[("id", &exchange)]).send().await?,
            Pass::Slug => {
                rb.query(&[("slug", &exchange.to_lowercase())])
                    .send()
                    .await?
            }
            Pass::Address => return Err(CmcErrors::PassIncompatible),
        };

        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<ExchangeMetadata>().await?;
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

    /// Returns a paginated list of all active cryptocurrency exchanges by CoinMarketCap ID.
    ///
    /// # Examples:
    ///
    /// Parameters:
    ///
    /// - `listing_status`:
    ///
    ///  **Active**: Only active exchanges are returned.
    ///
    ///  **Inactive**: List of exchanges that are no longer active.
    ///
    ///  **Untracked**: List of exchanges that are registered but do not currently meet methodology requirements to have active markets tracked.
    ///
    /// - `start`: Optionally offset the start (1-based index) of the paginated list of items to return.
    ///
    /// - `limit`: Optionally specify the number of results to return. Use this parameter and the "start" parameter to determine your own pagination size.
    ///
    /// - `sort`: What field to sort the list of exchanges by.
    ///
    /// - `crypto_id`: Optionally include one fiat or cryptocurrency IDs to filter market pairs by.
    ///
    /// ```rust
    /// use cmc::{Cmc, ListingStatusExchange, SortExchange};
    ///
    /// let cmc = Cmc::new("<API KEY>");
    ///
    /// match cmc.exchange_id_map(ListingStatusExchange::Active, 1, 10, SortExchange::Id, None) {
    ///     Ok(map) => println!("{}", map),
    ///     Err(err) => println!("{}", err),
    /// }
    /// ```
    #[cfg(feature = "exchange")]
    pub async fn exchange_id_map(
        &self,
        listing_status: ListingStatusExchange,
        start: usize,
        limit: usize,
        sort: SortExchange,
        crypto_id: Option<&str>,
    ) -> CmcResult<CmcExchangeIdMap> {
        let rb = self
            .add_endpoint("v1/exchange/map")
            .query(&[("start", start), ("limit", limit)]);

        let rb = match listing_status {
            ListingStatusExchange::Active => rb.query(&[("listing_status", "active")]),
            ListingStatusExchange::Inactive => rb.query(&[("listing_status", "inactive")]),
            ListingStatusExchange::Untracked => rb.query(&[("listing_status", "untracked")]),
        };

        let rb = match sort {
            SortExchange::Id => rb.query(&[("sort", "id")]),
            SortExchange::Volume24h => rb.query(&[("sort", "volume_24h")]),
        };

        let resp = if let Some(id) = crypto_id {
            rb.query(&[("crypto_id", id)]).send().await?
        } else {
            rb.send().await?
        };

        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<CmcExchangeIdMap>().await?;
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
}
