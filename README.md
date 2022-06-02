# cmc

### Unofficial Rust Library for the [CoinMarketCap API](https://coinmarketcap.com/api/)





## Usage



### Get your API key [here](https://coinmarketcap.com/api/)




### Price cryptocurrency
```rust
 use cmc::Cmc;
 let cmc = Cmc::new("<your api key>");
 match cmc.price("BTC") {
     Ok(price) => println!("{}", price),
     Err(err) => println!("Error: {}", err),
 }
```
 ### Price with custom settings
```rust
 use cmc::{CmcBuilder, Pass};
 let cmc = CmcBuilder::new("<your api key>").pass(Pass::Id).build();
 match cmc.price("1027") { // 1027 is Ethereum id.
     Ok(price) => println!("{}", price),
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
