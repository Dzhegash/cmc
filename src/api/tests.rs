#[cfg(test)]
mod deserialize_tests {
    use crate::api::cryptocurrency::quotes_latest_v2::*;

    #[test]
    fn deserialize_struct_id() {
        let btc_id = "1";
        let raw = r#"{"status":{"timestamp":"2022-06-16T07:21:13.621Z","error_code":0,"error_message":null,"elapsed":13,"credit_count":1,"notice":null},"data":{"1":{"id":1,"name":"Bitcoin","symbol":"BTC","slug":"bitcoin","num_market_pairs":9534,"date_added":null,"tags":[{"slug":"mineable","name":"Mineable","category":"OTHERS"},{"slug":"pow","name":"PoW","category":"ALGORITHM"},{"slug":"sha-256","name":"SHA-256","category":"ALGORITHM"},{"slug":"store-of-value","name":"Store Of Value","category":"CATEGORY"},{"slug":"state-channel","name":"State Channel","category":"CATEGORY"},{"slug":"coinbase-ventures-portfolio","name":"Coinbase Ventures Portfolio","category":"CATEGORY"},{"slug":"three-arrows-capital-portfolio","name":"Three Arrows Capital Portfolio","category":"CATEGORY"},{"slug":"polychain-capital-portfolio","name":"Polychain Capital Portfolio","category":"CATEGORY"},{"slug":"binance-labs-portfolio","name":"Binance Labs Portfolio","category":"CATEGORY"},{"slug":"blockchain-capital-portfolio","name":"Blockchain Capital Portfolio","category":"CATEGORY"},{"slug":"boostvc-portfolio","name":"BoostVC Portfolio","category":"CATEGORY"},{"slug":"cms-holdings-portfolio","name":"CMS Holdings Portfolio","category":"CATEGORY"},{"slug":"dcg-portfolio","name":"DCG Portfolio","category":"CATEGORY"},{"slug":"dragonfly-capital-portfolio","name":"DragonFly Capital Portfolio","category":"CATEGORY"},{"slug":"electric-capital-portfolio","name":"Electric Capital Portfolio","category":"CATEGORY"},{"slug":"fabric-ventures-portfolio","name":"Fabric Ventures Portfolio","category":"CATEGORY"},{"slug":"framework-ventures-portfolio","name":"Framework Ventures Portfolio","category":"CATEGORY"},{"slug":"galaxy-digital-portfolio","name":"Galaxy Digital Portfolio","category":"CATEGORY"},{"slug":"huobi-capital-portfolio","name":"Huobi Capital Portfolio","category":"CATEGORY"},{"slug":"alameda-research-portfolio","name":"Alameda Research Portfolio","category":"CATEGORY"},{"slug":"a16z-portfolio","name":"a16z Portfolio","category":"CATEGORY"},{"slug":"1confirmation-portfolio","name":"1Confirmation Portfolio","category":"CATEGORY"},{"slug":"winklevoss-capital-portfolio","name":"Winklevoss Capital Portfolio","category":"CATEGORY"},{"slug":"usv-portfolio","name":"USV Portfolio","category":"CATEGORY"},{"slug":"placeholder-ventures-portfolio","name":"Placeholder Ventures Portfolio","category":"CATEGORY"},{"slug":"pantera-capital-portfolio","name":"Pantera Capital Portfolio","category":"CATEGORY"},{"slug":"multicoin-capital-portfolio","name":"Multicoin Capital Portfolio","category":"CATEGORY"},{"slug":"paradigm-portfolio","name":"Paradigm Portfolio","category":"CATEGORY"}],"max_supply":21000000,"circulating_supply":19068831,"total_supply":19068831,"is_active":1,"platform":null,"cmc_rank":1,"is_fiat":0,"self_reported_circulating_supply":null,"self_reported_market_cap":null,"tvl_ratio":null,"last_updated":"2022-06-16T07:18:00.000Z","quote":{"USD":{"price":21864.74930690019,"volume_24h":50466092571.5209,"volume_change_24h":21.2426,"percent_change_1h":-0.52635812,"percent_change_24h":2.66543925,"percent_change_7d":-27.94289474,"percent_change_30d":-28.03299897,"percent_change_60d":-45.81352464,"percent_change_90d":-46.33345689,"market_cap":416935209390.64685,"market_cap_dominance":44.3846,"fully_diluted_market_cap":459159735444.9,"tvl":null,"last_updated":"2022-06-16T07:18:00.000Z"}}}}}"#;
        let root: QuotesLatestV2SlugOrId = serde_json::from_str(raw).unwrap();
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
        let root: QuotesLatestV2SlugOrId = serde_json::from_str(raw).unwrap();
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
        let root: QuotesLatestV2Symbol = serde_json::from_str(raw).unwrap();
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
    use crate::{CmcBuilder, Pass};
    use std::env;

    #[test]
    fn net_price() {
        let apikey = env::var("CMC_API").unwrap();
        use crate::Cmc;

        let cmc = Cmc::new(apikey);
        assert!(cmc.price("BTC").unwrap() > 0.1)
    }

    #[test]
    fn net_price_id() {
        let apikey = env::var("CMC_API").unwrap();

        let cmc = CmcBuilder::new(apikey)
            .pass(Pass::Id)
            .convert("EUR")
            .build();

        assert!(cmc.price("1027").unwrap() > 0.1);
    }

    #[test]
    fn net_price_slug() {
        let apikey = env::var("CMC_API").unwrap();

        let cmc = CmcBuilder::new(apikey)
            .pass(Pass::Slug)
            .convert("EUR")
            .build();

        assert!(cmc.price("bitcoin").unwrap() > 0.1);
    }

    #[test]
    fn net_key_info() {
        let apikey = env::var("CMC_API").unwrap();
        use crate::Cmc;

        let cmc = Cmc::new(apikey);
        let key_info = cmc.key_info().unwrap();
        assert!(key_info.plan.credit_limit_daily > 0);
    }
}
