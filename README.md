# cmc


[![Crates.io](https://img.shields.io/crates/v/cmc)](https://crates.io/crates/cmc)
[![docs.rs](https://img.shields.io/docsrs/cmc)](https://docs.rs/crate/cmc/latest)
[![MIT licensed](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE-MIT)
[![Apache-2.0 licensed](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE-APACHE)

##### Unofficial Rust Library for the [CoinMarketCap API](https://coinmarketcap.com/api/)

## Usage


### Get your API key [here](https://coinmarketcap.com/api/)
___

#### CoinMarketCap ID Map
**NOTE**: `CoinMarketCap recommend utilizing CMC ID instead of cryptocurrency symbols to securely identify cryptocurrencies with other endpoints and in your own application logic.`
```rust
use cmc::{Cmc, Sort};

let cmc = Cmc::new("<API KEY>");

match cmc.id_map(1, 50, Sort::CmcRank) {
    Ok(map) => println!("{}", map),
    Err(err) => println!("{}", err),
}
```

#### Price cryptocurrency
```rust
use cmc::Cmc;

let cmc = Cmc::new("<API KEY>");

match cmc.price("BTC") {
    Ok(price) => println!("Price: {}", price),
    Err(err) => println!("Error: {}", err),
}
```

#### Price with custom settings

```rust
use cmc::{CmcBuilder, Pass};

let cmc = CmcBuilder::new("<API KEY>")
    .pass(Pass::Id)
    .convert("EUR")
    .build();

match cmc.price("1027") { // 1027 is Ethereum id.
    Ok(price) => println!("Price: {}", price), // In Euro instead default USD
    Err(err) => println!("Error: {}", err),
}
```

#### Price conversion

```rust
use cmc::Cmc;

let cmc = Cmc::new("<API KEY>");

// 2.5 BTC in EUR (using symbols)
match cmc.price_conversion(2.5, "BTC", None, "EUR") {
    Ok(price) => println!("Total price: {}", price),
    Err(err) => println!("Error: {}", err),
}

// 1.6 ETH in XMR (using id's)
match cmc.price_conversion_id(1.6, "1027", None, "328") {
    Ok(price) => println!("Total price: {}", price),
    Err(err) => println!("Error: {}", err),
}
```

#### Exchange ID Map

```rust
use cmc::{Cmc, ListingStatusExchange, SortExchange};

let cmc = Cmc::new("<API KEY>");

match cmc.exchange_id_map(ListingStatusExchange::Active, 1, 10, SortExchange::Id, None) {
    Ok(map) => println!("{}", map),
    Err(err) => println!("{}", err),
}
```
___
## Crate Features
This crate supports default features:
- `cryptocurrency`
- `exchange`
- `fiat`
- `global_metrics`
- `key`
- `tools`


Disable all functions except the necessary ones:
```toml
[dependencies]
cmc = { version = "0.4.0", default-features = false, features = ["cryptocurrency"] }
```
## Async
Asynchronous versions of functions are available through enabling the async feature:
```toml
[dependencies]
cmc = { version = "0.4.0", features = ["async"] }
```
And then the code:
```rust
#[tokio::main]
async fn main() {
    use cmc::async_api::Cmc;
    
    let cmc = Cmc::new("<API KEY>");
    
    match cmc.price("BTC").await {
        Ok(price) => println!("Price: {}", price),
        Err(err) => println!("Error: {}", err),
    }
}
```
___


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
