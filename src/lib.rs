//! # cmc
//!
//! The `cmc` is an unofficial library for [`CoinMarketCap API`][coinmarketcap]
//!
//!## Get price cryptocurrency
//!```rust
//! use cmc::Cmc;
//!
//! let cmc = Cmc::new("<your API key>");
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
//! let cmc = CmcBuilder::new("<your API key>").pass(Pass::Id).build();
//! match cmc.price("1027") { // 1027 is Ethereum id.
//!     Ok(price) => println!("{}", price),
//!     Err(err) => println!("Error: {}", err),
//! }
//!```
//! [coinmarketcap]: https://coinmarketcap.com/api/

pub use self::api::{Cmc, CmcBuilder, Pass};
pub mod api;
mod errors;
