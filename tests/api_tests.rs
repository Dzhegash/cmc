#[cfg(test)]
mod deserialize_tests {
    use cmc::api::cryptocurrency::quotes_latest_v2::*;

    #[test]
    fn deserialize_struct_id() {
        let btc_id = "1";
        let raw = r#"{"status":{"timestamp":"2022-06-16T07:21:13.621Z","error_code":0,"error_message":null,"elapsed":13,"credit_count":1,"notice":null},"data":{"1":{"id":1,"name":"Bitcoin","symbol":"BTC","slug":"bitcoin","num_market_pairs":9534,"date_added":null,"tags":[{"slug":"mineable","name":"Mineable","category":"OTHERS"},{"slug":"pow","name":"PoW","category":"ALGORITHM"},{"slug":"sha-256","name":"SHA-256","category":"ALGORITHM"},{"slug":"store-of-value","name":"Store Of Value","category":"CATEGORY"},{"slug":"state-channel","name":"State Channel","category":"CATEGORY"},{"slug":"coinbase-ventures-portfolio","name":"Coinbase Ventures Portfolio","category":"CATEGORY"},{"slug":"three-arrows-capital-portfolio","name":"Three Arrows Capital Portfolio","category":"CATEGORY"},{"slug":"polychain-capital-portfolio","name":"Polychain Capital Portfolio","category":"CATEGORY"},{"slug":"binance-labs-portfolio","name":"Binance Labs Portfolio","category":"CATEGORY"},{"slug":"blockchain-capital-portfolio","name":"Blockchain Capital Portfolio","category":"CATEGORY"},{"slug":"boostvc-portfolio","name":"BoostVC Portfolio","category":"CATEGORY"},{"slug":"cms-holdings-portfolio","name":"CMS Holdings Portfolio","category":"CATEGORY"},{"slug":"dcg-portfolio","name":"DCG Portfolio","category":"CATEGORY"},{"slug":"dragonfly-capital-portfolio","name":"DragonFly Capital Portfolio","category":"CATEGORY"},{"slug":"electric-capital-portfolio","name":"Electric Capital Portfolio","category":"CATEGORY"},{"slug":"fabric-ventures-portfolio","name":"Fabric Ventures Portfolio","category":"CATEGORY"},{"slug":"framework-ventures-portfolio","name":"Framework Ventures Portfolio","category":"CATEGORY"},{"slug":"galaxy-digital-portfolio","name":"Galaxy Digital Portfolio","category":"CATEGORY"},{"slug":"huobi-capital-portfolio","name":"Huobi Capital Portfolio","category":"CATEGORY"},{"slug":"alameda-research-portfolio","name":"Alameda Research Portfolio","category":"CATEGORY"},{"slug":"a16z-portfolio","name":"a16z Portfolio","category":"CATEGORY"},{"slug":"1confirmation-portfolio","name":"1Confirmation Portfolio","category":"CATEGORY"},{"slug":"winklevoss-capital-portfolio","name":"Winklevoss Capital Portfolio","category":"CATEGORY"},{"slug":"usv-portfolio","name":"USV Portfolio","category":"CATEGORY"},{"slug":"placeholder-ventures-portfolio","name":"Placeholder Ventures Portfolio","category":"CATEGORY"},{"slug":"pantera-capital-portfolio","name":"Pantera Capital Portfolio","category":"CATEGORY"},{"slug":"multicoin-capital-portfolio","name":"Multicoin Capital Portfolio","category":"CATEGORY"},{"slug":"paradigm-portfolio","name":"Paradigm Portfolio","category":"CATEGORY"}],"max_supply":21000000,"circulating_supply":19068831,"total_supply":19068831,"is_active":1,"platform":null,"cmc_rank":1,"is_fiat":0,"self_reported_circulating_supply":null,"self_reported_market_cap":null,"tvl_ratio":null,"last_updated":"2022-06-16T07:18:00.000Z","quote":{"USD":{"price":21864.74930690019,"volume_24h":50466092571.5209,"volume_change_24h":21.2426,"percent_change_1h":-0.52635812,"percent_change_24h":2.66543925,"percent_change_7d":-27.94289474,"percent_change_30d":-28.03299897,"percent_change_60d":-45.81352464,"percent_change_90d":-46.33345689,"market_cap":416935209390.64685,"market_cap_dominance":44.3846,"fully_diluted_market_cap":459159735444.9,"tvl":null,"last_updated":"2022-06-16T07:18:00.000Z"}}}}}"#;
        let root: QLv2Slug = serde_json::from_str(raw).unwrap();
        let price = root
            .data
            .get(btc_id)
            .unwrap()
            .quote
            .get("USD")
            .unwrap()
            .price;

        assert_eq!(price, 21864.74930690019);
    }

    #[test]
    fn deserialize_struct_slug() {
        let raw = r#"{"status":{"timestamp":"2022-06-16T07:23:14.863Z","error_code":0,"error_message":null,"elapsed":17,"credit_count":1,"notice":null},"data":{"1":{"id":1,"name":"Bitcoin","symbol":"BTC","slug":"bitcoin","num_market_pairs":9534,"date_added":null,"tags":[{"slug":"mineable","name":"Mineable","category":"OTHERS"},{"slug":"pow","name":"PoW","category":"ALGORITHM"},{"slug":"sha-256","name":"SHA-256","category":"ALGORITHM"},{"slug":"store-of-value","name":"Store Of Value","category":"CATEGORY"},{"slug":"state-channel","name":"State Channel","category":"CATEGORY"},{"slug":"coinbase-ventures-portfolio","name":"Coinbase Ventures Portfolio","category":"CATEGORY"},{"slug":"three-arrows-capital-portfolio","name":"Three Arrows Capital Portfolio","category":"CATEGORY"},{"slug":"polychain-capital-portfolio","name":"Polychain Capital Portfolio","category":"CATEGORY"},{"slug":"binance-labs-portfolio","name":"Binance Labs Portfolio","category":"CATEGORY"},{"slug":"blockchain-capital-portfolio","name":"Blockchain Capital Portfolio","category":"CATEGORY"},{"slug":"boostvc-portfolio","name":"BoostVC Portfolio","category":"CATEGORY"},{"slug":"cms-holdings-portfolio","name":"CMS Holdings Portfolio","category":"CATEGORY"},{"slug":"dcg-portfolio","name":"DCG Portfolio","category":"CATEGORY"},{"slug":"dragonfly-capital-portfolio","name":"DragonFly Capital Portfolio","category":"CATEGORY"},{"slug":"electric-capital-portfolio","name":"Electric Capital Portfolio","category":"CATEGORY"},{"slug":"fabric-ventures-portfolio","name":"Fabric Ventures Portfolio","category":"CATEGORY"},{"slug":"framework-ventures-portfolio","name":"Framework Ventures Portfolio","category":"CATEGORY"},{"slug":"galaxy-digital-portfolio","name":"Galaxy Digital Portfolio","category":"CATEGORY"},{"slug":"huobi-capital-portfolio","name":"Huobi Capital Portfolio","category":"CATEGORY"},{"slug":"alameda-research-portfolio","name":"Alameda Research Portfolio","category":"CATEGORY"},{"slug":"a16z-portfolio","name":"a16z Portfolio","category":"CATEGORY"},{"slug":"1confirmation-portfolio","name":"1Confirmation Portfolio","category":"CATEGORY"},{"slug":"winklevoss-capital-portfolio","name":"Winklevoss Capital Portfolio","category":"CATEGORY"},{"slug":"usv-portfolio","name":"USV Portfolio","category":"CATEGORY"},{"slug":"placeholder-ventures-portfolio","name":"Placeholder Ventures Portfolio","category":"CATEGORY"},{"slug":"pantera-capital-portfolio","name":"Pantera Capital Portfolio","category":"CATEGORY"},{"slug":"multicoin-capital-portfolio","name":"Multicoin Capital Portfolio","category":"CATEGORY"},{"slug":"paradigm-portfolio","name":"Paradigm Portfolio","category":"CATEGORY"}],"max_supply":21000000,"circulating_supply":19068831,"total_supply":19068831,"is_active":1,"platform":null,"cmc_rank":1,"is_fiat":0,"self_reported_circulating_supply":null,"self_reported_market_cap":null,"tvl_ratio":null,"last_updated":"2022-06-16T07:22:00.000Z","quote":{"USD":{"price":21847.757042255787,"volume_24h":50520517177.985275,"volume_change_24h":21.5421,"percent_change_1h":-0.60874824,"percent_change_24h":2.66745076,"percent_change_7d":-28.05069885,"percent_change_30d":-28.00942096,"percent_change_60d":-45.8775093,"percent_change_90d":-46.3309831,"market_cap":416611186767.83545,"market_cap_dominance":44.3513,"fully_diluted_market_cap":458802897887.37,"tvl":null,"last_updated":"2022-06-16T07:22:00.000Z"}}}}}"#;
        let root: QLv2Slug = serde_json::from_str(raw).unwrap();
        let slug_id = root.data.iter().next().unwrap().0;
        let price = root
            .data
            .get(slug_id)
            .unwrap()
            .quote
            .get("USD")
            .unwrap()
            .price;

        assert_eq!(price, 21847.757042255787);
    }

    #[test]
    fn deserialize_struct_symbol() {
        let raw = r#"{"status":{"timestamp":"2022-06-16T07:11:29.938Z","error_code":0,"error_message":null,"elapsed":15,"credit_count":1,"notice":null},"data":{"BTC":[{"id":1,"name":"Bitcoin","symbol":"BTC","slug":"bitcoin","num_market_pairs":9534,"date_added":null,"tags":[{"slug":"mineable","name":"Mineable","category":"OTHERS"},{"slug":"pow","name":"PoW","category":"ALGORITHM"},{"slug":"sha-256","name":"SHA-256","category":"ALGORITHM"},{"slug":"store-of-value","name":"Store Of Value","category":"CATEGORY"},{"slug":"state-channel","name":"State Channel","category":"CATEGORY"},{"slug":"coinbase-ventures-portfolio","name":"Coinbase Ventures Portfolio","category":"CATEGORY"},{"slug":"three-arrows-capital-portfolio","name":"Three Arrows Capital Portfolio","category":"CATEGORY"},{"slug":"polychain-capital-portfolio","name":"Polychain Capital Portfolio","category":"CATEGORY"},{"slug":"binance-labs-portfolio","name":"Binance Labs Portfolio","category":"CATEGORY"},{"slug":"blockchain-capital-portfolio","name":"Blockchain Capital Portfolio","category":"CATEGORY"},{"slug":"boostvc-portfolio","name":"BoostVC Portfolio","category":"CATEGORY"},{"slug":"cms-holdings-portfolio","name":"CMS Holdings Portfolio","category":"CATEGORY"},{"slug":"dcg-portfolio","name":"DCG Portfolio","category":"CATEGORY"},{"slug":"dragonfly-capital-portfolio","name":"DragonFly Capital Portfolio","category":"CATEGORY"},{"slug":"electric-capital-portfolio","name":"Electric Capital Portfolio","category":"CATEGORY"},{"slug":"fabric-ventures-portfolio","name":"Fabric Ventures Portfolio","category":"CATEGORY"},{"slug":"framework-ventures-portfolio","name":"Framework Ventures Portfolio","category":"CATEGORY"},{"slug":"galaxy-digital-portfolio","name":"Galaxy Digital Portfolio","category":"CATEGORY"},{"slug":"huobi-capital-portfolio","name":"Huobi Capital Portfolio","category":"CATEGORY"},{"slug":"alameda-research-portfolio","name":"Alameda Research Portfolio","category":"CATEGORY"},{"slug":"a16z-portfolio","name":"a16z Portfolio","category":"CATEGORY"},{"slug":"1confirmation-portfolio","name":"1Confirmation Portfolio","category":"CATEGORY"},{"slug":"winklevoss-capital-portfolio","name":"Winklevoss Capital Portfolio","category":"CATEGORY"},{"slug":"usv-portfolio","name":"USV Portfolio","category":"CATEGORY"},{"slug":"placeholder-ventures-portfolio","name":"Placeholder Ventures Portfolio","category":"CATEGORY"},{"slug":"pantera-capital-portfolio","name":"Pantera Capital Portfolio","category":"CATEGORY"},{"slug":"multicoin-capital-portfolio","name":"Multicoin Capital Portfolio","category":"CATEGORY"},{"slug":"paradigm-portfolio","name":"Paradigm Portfolio","category":"CATEGORY"}],"max_supply":21000000,"circulating_supply":19068831,"total_supply":19068831,"is_active":1,"platform":null,"cmc_rank":1,"is_fiat":0,"self_reported_circulating_supply":null,"self_reported_market_cap":null,"tvl_ratio":null,"last_updated":"2022-06-16T07:10:00.000Z","quote":{"USD":{"price":21871.88352845583,"volume_24h":50482520018.50814,"volume_change_24h":20.8557,"percent_change_1h":-0.39064163,"percent_change_24h":2.53462368,"percent_change_7d":-27.88511067,"percent_change_30d":-28.15731968,"percent_change_60d":-45.79275474,"percent_change_90d":-46.37359146,"market_cap":417071250655.8079,"market_cap_dominance":44.4243,"fully_diluted_market_cap":459309554097.57,"tvl":null,"last_updated":"2022-06-16T07:10:00.000Z"}}}]}}"#;
        let root: QLv2Symbol = serde_json::from_str(raw).unwrap();
        let price = root.data.get("BTC").unwrap()[0]
            .quote
            .get("USD")
            .unwrap()
            .price;

        assert_eq!(price, 21871.88352845583);
    }
}

#[cfg(test)]
mod network_tests {
    use cmc::{Cmc, CmcBuilder, ListingStatusExchange, Pass, Sort, SortExchange, SortFiat};
    const APIKEY: &str = env!("CMC_API");

    #[test]
    fn net_price() {
        let cmc = Cmc::new(APIKEY);
        assert!(cmc.price("BTC").unwrap() > 0.1)
    }

    #[test]
    fn net_price_id() {
        let cmc = CmcBuilder::new(APIKEY)
            .pass(Pass::Id)
            .convert("EUR")
            .build();

        assert!(cmc.price("1027").unwrap() > 0.1);
    }

    #[test]
    fn net_price_slug() {
        let cmc = CmcBuilder::new(APIKEY)
            .pass(Pass::Slug)
            .convert("EUR")
            .build();

        assert!(cmc.price("bitcoin").unwrap() > 0.1);
    }

    #[test]
    fn net_key_info() {
        let cmc = Cmc::new(APIKEY);
        let key_info = cmc.key_info().unwrap();
        assert!(key_info.plan.credit_limit_monthly > 0);
    }

    #[test]
    fn net_id_map() {
        let cmc = Cmc::new(APIKEY);
        let map = cmc.id_map(1, 10, Sort::CmcRank).unwrap();
        assert_eq!(map.data[1].id, 1027);
        assert_eq!(map.data[1].slug, "ethereum");
    }

    #[test]
    fn net_fiat_id_map() {
        let cmc = Cmc::new(APIKEY);
        let map = cmc.fiat_id_map(1, 10, SortFiat::Id).unwrap();
        assert_eq!(map.data[0].symbol, "USD");
    }

    #[test]
    fn net_price_conversion() {
        let cmc = Cmc::new(APIKEY);
        let price = cmc.price_conversion(2.5, "BTC", None, "usd").unwrap();
        assert!(price > 0.1);
    }

    #[test]
    fn net_price_conversion_id() {
        let cmc = Cmc::new(APIKEY);
        let price = cmc.price_conversion_id(2.5, "1", None, "2781").unwrap();
        assert!(price > 0.1);
    }

    #[test]
    fn net_categories() {
        let cmc = CmcBuilder::new(APIKEY).pass(Pass::Id).build();
        let categories = cmc.categories(1, 10, "1027").unwrap();
        assert!(categories.data[0].market_cap > 0.1)
    }

    #[test]
    fn net_category() {
        let cmc = CmcBuilder::new(APIKEY).convert_id("1027").build();
        let category = cmc.category("605e2ce9d41eae1066535f7c", 1, 10).unwrap();
        assert!(category.volume > 0.1)
    }

    #[test]
    fn net_metadata_id() {
        let cmc = CmcBuilder::new(APIKEY).pass(Pass::Id).build();
        let name = cmc.metadata("1027").unwrap().name;
        assert_eq!("Ethereum", name);
    }

    #[test]
    fn net_metadata_symbol() {
        let cmc = CmcBuilder::new(APIKEY).pass(Pass::Symbol).build();
        let name = cmc.metadata("ETH").unwrap().name;
        assert_eq!("Ethereum", name);
    }

    #[test]
    fn net_metadata_slug() {
        let cmc = CmcBuilder::new(APIKEY).pass(Pass::Slug).build();
        let name = cmc.metadata("ethereum").unwrap().name;
        assert_eq!("Ethereum", name);
    }

    #[test]
    fn net_metadata_address() {
        let cmc = CmcBuilder::new(APIKEY).pass(Pass::Address).build();
        let id = cmc
            .metadata("0xc40af1e4fecfa05ce6bab79dcd8b373d2e436c4e")
            .unwrap()
            .id;
        assert_eq!(9458, id);
    }

    #[test]
    fn net_global_metrics_convert() {
        let cmc = CmcBuilder::new(APIKEY).convert("ETH").build();

        let active_cc = cmc.global_metrics().unwrap().active_cryptocurrencies;

        assert!(active_cc > 0);
    }

    #[test]
    fn net_global_metrics_convert_id() {
        let cmc = CmcBuilder::new(APIKEY).convert_id("1027").build();

        let total = cmc
            .global_metrics()
            .unwrap()
            .quote
            .get("1027")
            .unwrap()
            .total_market_cap;

        assert!(total > 1.0);
    }

    #[test]
    fn net_exchange_metadata_id() {
        let cmc = CmcBuilder::new(APIKEY).pass(Pass::Id).build();

        let metadata = cmc.exchange_metadata("270").unwrap().data;

        let name = &metadata.get("270").unwrap().name;

        assert_eq!(name, "Binance");
    }

    #[test]
    fn net_exchange_metadata_slug() {
        let cmc = CmcBuilder::new(APIKEY).pass(Pass::Slug).build();

        let metadata = cmc.exchange_metadata("binance").unwrap().data;

        let name = &metadata.get("binance").unwrap().name;

        assert_eq!(name, "Binance");
    }

    #[test]
    fn net_quotes_latest_by_id() {
        let cmc = Cmc::new(APIKEY);
        let price = cmc
            .quotes_latest_by_id("1027")
            .unwrap()
            .data
            .get("1027")
            .unwrap()
            .quote
            .get("USD")
            .unwrap()
            .price;

        assert!(price > 0.1);
    }

    #[test]
    fn net_quotes_latest_by_slug() {
        let cmc = Cmc::new(APIKEY);
        let price = cmc
            .quotes_latest_by_slug("ethereum")
            .unwrap()
            .data
            .get("1027")
            .unwrap()
            .quote
            .get("USD")
            .unwrap()
            .price;

        assert!(price > 0.1);
    }

    #[test]
    fn net_quotes_latest_by_symbol() {
        let cmc = Cmc::new(APIKEY);
        let price = cmc
            .quotes_latest_by_symbol("ETH")
            .unwrap()
            .data
            .get("ETH")
            .unwrap()[0]
            .quote
            .get("USD")
            .unwrap()
            .price;

        assert!(price > 0.1);
    }

    #[test]
    fn net_exchange_id_map() {
        let cmc = Cmc::new(APIKEY);
        let id = cmc
            .exchange_id_map(ListingStatusExchange::Active, 1, 10, SortExchange::Id, None)
            .unwrap()
            .data[0]
            .id;

        assert_eq!(16, id);
    }
}
