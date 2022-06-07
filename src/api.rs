use crate::api::cryptocurrency::quotes_latest_v2::{QuotesLatestV2SlugOrId, QuotesLatestV2Symbol};
use crate::errors::CmcErrors;
use api_errors::ApiError;
use reqwest::blocking::{Client, RequestBuilder};
use reqwest::StatusCode;

pub mod api_errors;
mod cryptocurrency;
mod fiat;
mod tests;

const CMC_API_URL: &str = "https://pro-api.coinmarketcap.com/";
type CmcResult<T> = Result<T, CmcErrors>;

type RootIdMap = cryptocurrency::coinmarketcap_id_map::CoinMarketCapIdMap;
type IdMap = Vec<cryptocurrency::coinmarketcap_id_map::Cryptocurrency>;
type RootIdMapFiat = fiat::coinmarketcap_id_map::CoinMarketCapIdMap;
type IdMapFiat = Vec<fiat::coinmarketcap_id_map::Currency>;

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
    /// match cmc.price("1027") { // 1027 is Ethereum id.
    ///     Ok(price) => println!("{}", price),
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
    /// match cmc.price("ETH") {
    ///     Ok(price) => println!("{}", price), // In Euro
    ///     Err(err) => println!("Error: {}", err),
    /// }
    /// ```
    pub fn convert<T: Into<String>>(mut self, currency: T) -> CmcBuilder {
        self.config.currency = currency.into().to_uppercase();
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
    /// let map = cmc.id_map(1, 5, Sort::Id).unwrap();
    ///
    /// for cc in map {
    ///     println!(
    ///         "CMC Id: {}\nName: {}\nSymbol: {}\nSlug: {}\nRank: {}\n---------------",
    ///         cc.id, cc.name, cc.symbol, cc.slug, cc.rank
    ///     )
    /// }
    /// ```
    pub fn id_map(&self, start: usize, limit: usize, sort: Sort) -> CmcResult<IdMap> {
        let resp = match sort {
            Sort::Id => self
                .add_endpoint("v1/cryptocurrency/map")
                .query(&[("start", start), ("limit", limit)])
                .query(&[("sort", "id")])
                .send()?,
            Sort::CmcRank => self
                .add_endpoint("v1/cryptocurrency/map")
                .query(&[("start", start), ("limit", limit)])
                .query(&[("sort", "cmc_rank")])
                .send()?,
        };

        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<RootIdMap>()?;
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

    /// Latest price for cryptocurrency in USD.
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use cmc::Cmc;
    ///
    /// let cmc = Cmc::new("<API KEY>");
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
            .query(&[("id", id), ("convert", currency)])
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
                let root = resp.json::<ApiError>()?;
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
            .query(&[
                ("slug", slug.to_lowercase().as_str()),
                ("convert", currency),
            ])
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
                let root = resp.json::<ApiError>()?;
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
            .query(&[("symbol", symbol), ("convert", currency)])
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
    /// - `sort` What field to sort the list by.
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use cmc::{Cmc, SortFiat};
    ///
    /// let cmc = Cmc::new("<API KEY>");
    /// let map_fiat = cmc.id_map_fiat(1, 100, SortFiat::Name).unwrap();
    /// for f in map_fiat {
    ///     println!(
    ///         "Id: {}\nName: {}\nSign: {}\nSymbol: {}\n---------------",
    ///         f.id, f.name, f.sign, f.symbol
    ///     )
    /// }
    /// ```
    pub fn id_map_fiat(&self, start: usize, limit: usize, sort: SortFiat) -> CmcResult<IdMapFiat> {
        let resp = match sort {
            SortFiat::Id => self
                .add_endpoint("v1/fiat/map")
                .query(&[("start", start), ("limit", limit)])
                .query(&[("sort", "id")])
                .send()?,
            SortFiat::Name => self
                .add_endpoint("v1/fiat/map")
                .query(&[("start", start), ("limit", limit)])
                .query(&[("sort", "name")])
                .send()?,
        };

        match resp.status() {
            StatusCode::OK => {
                let root = resp.json::<RootIdMapFiat>()?;
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
}
