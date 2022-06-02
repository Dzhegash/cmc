# cmc

### Unofficial Rust Library for the [CoinMarketCap API](https://coinmarketcap.com/api/)

[![Crates.io](https://img.shields.io/crates/v/cmc)](https://crates.io/crates/cmc)
[![docs.rs](https://img.shields.io/docsrs/cmc)](https://docs.rs/crate/cmc/latest)
[![MIT licensed](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE-MIT)
[![Apache-2.0 licensed](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE-APACHE)



## Usage



### Get your API key [here](https://coinmarketcap.com/api/)




### Price cryptocurrency
```rust
 use cmc::Cmc;

 let cmc = Cmc::new("<API KEY>");

 match cmc.price("BTC") {
    Ok(price) => println!("{}", price),
    Err(err) => println!("Error: {}", err),
 }
```
 ### Price with custom settings
```rust
 use cmc::{CmcBuilder, Pass};

 let cmc = CmcBuilder::new("<API KEY>")
 	 .pass(Pass::Id)
 	 .convert("EUR")
 	 .build();

 match cmc.price("1027") { // 1027 is Ethereum id.
 	Ok(price) => println!("{}", price), // In Euro instead default USD
 	Err(err) => println!("Error: {}", err),
 }
```

​	

## License



Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
