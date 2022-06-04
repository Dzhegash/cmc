//! # cmc
//!
//! The `cmc` is an unofficial library for [CoinMarketCap API][coinmarketcap]
//!
//! ## Get CoinMarketCap ID Map
//! **NOTE**: `CoinMarketCap recommend utilizing CMC ID instead of cryptocurrency symbols to securely identify cryptocurrencies with other endpoints and in your own application logic.`
//!```rust
//! use cmc::{Cmc, Sort};
//!
//! let cmc = Cmc::new("<API KEY>");
//! let map = cmc.id_map(1, 5, Sort::Id).unwrap();
//!
//! for cc in map {
//!     println!(
//!         "CMC Id: {}\nName: {}\nSymbol: {}\nSlug: {}\nRank: {}\n---------------",
//!         cc.id, cc.name, cc.symbol, cc.slug, cc.rank
//!     )
//! }
//!```
//!
//! ## Get price cryptocurrency
//!```rust
//! use cmc::Cmc;
//!
//! let cmc = Cmc::new("<API KEY>");
//!
//! match cmc.price("BTC") {
//!     Ok(price) => println!("{}", price),
//!     Err(err) => println!("Error: {}", err),
//! }
//!```
//!
//! ## Get price with custom settings
//!```rust
//! use cmc::{CmcBuilder, Pass};
//!
//! let cmc = CmcBuilder::new("<API KEY>")
//!     .pass(Pass::Id)
//!     .convert("EUR")
//!     .build();
//!
//! match cmc.price("1027") { // 1027 is Ethereum id.
//!     Ok(price) => println!("{}", price), // In Euro instead default USD
//!     Err(err) => println!("Error: {}", err),
//! }
//!```
//! [coinmarketcap]: https://coinmarketcap.com/api/

pub use self::api::{Cmc, CmcBuilder, Pass, Sort};
pub mod api;
mod errors;
