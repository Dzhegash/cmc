//! # cmc
//!
//! The `cmc` is an unofficial library for [CoinMarketCap API][coinmarketcap]
//!
//! ## CoinMarketCap ID Map
//! **NOTE**: `CoinMarketCap recommend utilizing CMC ID instead of cryptocurrency symbols to securely identify cryptocurrencies with other endpoints and in your own application logic.`
//!```rust
//! use cmc::{Cmc, Sort};
//!
//! let cmc = Cmc::new("<API KEY>");
//!
//! match cmc.id_map(1, 50, Sort::CmcRank) {
//!     Ok(map) => println!("{}", map),
//!     Err(err) => println!("{}", err),
//! }
//!```
//!
//! ## Price cryptocurrency
//!```rust
//! use cmc::Cmc;
//!
//! let cmc = Cmc::new("<API KEY>");
//!
//! match cmc.price("BTC") {
//!     Ok(price) => println!("Price: {}", price),
//!     Err(err) => println!("Error: {}", err),
//! }
//!```
//!
//! ## Price with custom settings
//!```rust
//! use cmc::{CmcBuilder, Pass};
//!
//! let cmc = CmcBuilder::new("<API KEY>")
//!     .pass(Pass::Id)
//!     .convert("EUR")
//!     .build();
//!
//! match cmc.price("1027") { // 1027 is Ethereum id.
//!     Ok(price) => println!("Price: {}", price), // In Euro instead default USD
//!     Err(err) => println!("Error: {}", err),
//! }
//!```
//!
//! ## Price conversion
//!```rust
//! use cmc::Cmc;
//!
//! let cmc = Cmc::new("<API KEY>");
//!
//! // 2.5 BTC in EUR (using symbols)
//! match cmc.price_conversion(2.5, "BTC", None, "EUR") {
//!     Ok(price) => println!("Total price: {}", price),
//!     Err(err) => println!("Error: {}", err),
//! }
//!
//! // 1.6 ETH in XMR (using id's)
//! match cmc.price_conversion_id(1.6, "1027", None, "328") {
//!     Ok(price) => println!("Total price: {}", price),
//!     Err(err) => println!("Error: {}", err),
//! }
//!```
//!
//! ## Exchange ID Map
//!```rust
//! use cmc::{Cmc, ListingStatusExchange, SortExchange};
//!
//! let cmc = Cmc::new("<API KEY>");
//!
//! match cmc.exchange_id_map(ListingStatusExchange::Active, 1, 10, SortExchange::Id, None) {
//!     Ok(map) => println!("{}", map),
//!     Err(err) => println!("{}", err),
//! }
//!```
//! [coinmarketcap]: https://coinmarketcap.com/api/
//!
//! ## Crate Features
//! This crate supports default features:
//!
//! - `cryptocurrency`
//! - `exchange`
//! - `fiat`
//! - `global_metrics`
//! - `key`
//! - `tools`
//!
//! Disable all functions except the necessary ones:
//!```toml
//! [dependencies]
//! cmc = { version = "0.4.0", default-features = false, features = ["cryptocurrency"] }
//!```
//! ## Async
//! Asynchronous versions of functions are available through enabling the async feature:
//! ```toml
//! [dependencies]
//! cmc = { version = "0.4.0", features = ["async"] }
//! ```
//! And then the code:
//! ```rust
//! #[tokio::main]
//! async fn main() {
//!     use cmc::async_api::Cmc;
//!
//!     let cmc = Cmc::new("<API KEY>");
//!
//!     match cmc.price("BTC").await {
//!         Ok(price) => println!("Price: {}", price),
//!         Err(err) => println!("Error: {}", err),
//!     }
//! }
//! ```

pub mod api;
#[cfg(any(feature = "async", doc))]
pub mod async_api;
pub mod errors;

#[doc(inline)]
pub use self::api::{Cmc, CmcBuilder, ListingStatusExchange, Pass, Sort, SortExchange, SortFiat};
