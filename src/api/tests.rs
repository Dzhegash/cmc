#[cfg(test)]
mod deserialize_tests {
    use crate::api::cryptocurrency::quotes_latest_v2::*;

    #[test]
    fn deserialize_struct_id() {
        let btc_id = "1";
        let raw = r#"{"status":{"timestamp":"2022-06-01T21:26:57.522Z","error_code":0,"error_message":null,"elapsed":32,"credit_count":1,"notice":null},"data":{"1":{"id":1,"name":"Bitcoin","symbol":"BTC","slug":"bitcoin","num_market_pairs":9477,"date_added":"2013-04-28T00:00:00.000Z","tags":[{"slug":"mineable","name":"Mineable","category":"OTHER"},{"slug":"pow","name":"PoW","category":"CONSENSUS_ALGORITHM"},{"slug":"sha-256","name":"SHA-256","category":"CONSENSUS_ALGORITHM"},{"slug":"store-of-value","name":"Store Of Value","category":"PROPERTY"},{"slug":"state-channel","name":"State Channel","category":"PROPERTY"},{"slug":"coinbase-ventures-portfolio","name":"Coinbase Ventures Portfolio","category":"PROPERTY"},{"slug":"three-arrows-capital-portfolio","name":"Three Arrows Capital Portfolio","category":"PROPERTY"},{"slug":"polychain-capital-portfolio","name":"Polychain Capital Portfolio","category":"PROPERTY"},{"slug":"binance-labs-portfolio","name":"Binance Labs Portfolio","category":"PROPERTY"},{"slug":"blockchain-capital-portfolio","name":"Blockchain Capital Portfolio","category":"PROPERTY"},{"slug":"boostvc-portfolio","name":"BoostVC Portfolio","category":"PROPERTY"},{"slug":"cms-holdings-portfolio","name":"CMS Holdings Portfolio","category":"PROPERTY"},{"slug":"dcg-portfolio","name":"DCG Portfolio","category":"PROPERTY"},{"slug":"dragonfly-capital-portfolio","name":"DragonFly Capital Portfolio","category":"PROPERTY"},{"slug":"electric-capital-portfolio","name":"Electric Capital Portfolio","category":"PROPERTY"},{"slug":"fabric-ventures-portfolio","name":"Fabric Ventures Portfolio","category":"PROPERTY"},{"slug":"framework-ventures-portfolio","name":"Framework Ventures Portfolio","category":"PROPERTY"},{"slug":"galaxy-digital-portfolio","name":"Galaxy Digital Portfolio","category":"PROPERTY"},{"slug":"huobi-capital-portfolio","name":"Huobi Capital Portfolio","category":"PROPERTY"},{"slug":"alameda-research-portfolio","name":"Alameda Research Portfolio","category":"PROPERTY"},{"slug":"a16z-portfolio","name":"a16z Portfolio","category":"PROPERTY"},{"slug":"1confirmation-portfolio","name":"1Confirmation Portfolio","category":"PROPERTY"},{"slug":"winklevoss-capital-portfolio","name":"Winklevoss Capital Portfolio","category":"PROPERTY"},{"slug":"usv-portfolio","name":"USV Portfolio","category":"PROPERTY"},{"slug":"placeholder-ventures-portfolio","name":"Placeholder Ventures Portfolio","category":"PROPERTY"},{"slug":"pantera-capital-portfolio","name":"Pantera Capital Portfolio","category":"PROPERTY"},{"slug":"multicoin-capital-portfolio","name":"Multicoin Capital Portfolio","category":"PROPERTY"},{"slug":"paradigm-portfolio","name":"Paradigm Portfolio","category":"PROPERTY"}],"max_supply":21000000,"circulating_supply":19055662,"total_supply":19055662,"is_active":1,"platform":null,"cmc_rank":1,"is_fiat":0,"self_reported_circulating_supply":null,"self_reported_market_cap":null,"last_updated":"2022-06-01T21:26:00.000Z","quote":{"USD":{"price":29705.93501696529,"volume_24h":39782528268.71518,"volume_change_24h":9.2595,"percent_change_1h":-0.98631865,"percent_change_24h":-6.48106206,"percent_change_7d":-0.18375664,"percent_change_30d":-22.83264803,"percent_change_60d":-35.56666606,"percent_change_90d":-29.38831116,"market_cap":566066257077.2549,"market_cap_dominance":46.0844,"fully_diluted_market_cap":623824635356.27,"last_updated":"2022-06-01T21:26:00.000Z"}}}}}"#;
        let root: QuotesLatestV2SlugOrId = serde_json::from_str(raw).unwrap();
        let price = root
            .data
            .get(btc_id)
            .unwrap()
            .quote
            .get("USD")
            .unwrap()
            .price;

        assert_eq!(price, 29705.93501696529);
    }

    #[test]
    fn deserialize_struct_slug() {
        let raw = r#"{"status":{"timestamp":"2022-06-01T21:30:22.944Z","error_code":0,"error_message":null,"elapsed":66,"credit_count":1,"notice":null},"data":{"1":{"id":1,"name":"Bitcoin","symbol":"BTC","slug":"bitcoin","num_market_pairs":9477,"date_added":"2013-04-28T00:00:00.000Z","tags":[{"slug":"mineable","name":"Mineable","category":"OTHER"},{"slug":"pow","name":"PoW","category":"CONSENSUS_ALGORITHM"},{"slug":"sha-256","name":"SHA-256","category":"CONSENSUS_ALGORITHM"},{"slug":"store-of-value","name":"Store Of Value","category":"PROPERTY"},{"slug":"state-channel","name":"State Channel","category":"PROPERTY"},{"slug":"coinbase-ventures-portfolio","name":"Coinbase Ventures Portfolio","category":"PROPERTY"},{"slug":"three-arrows-capital-portfolio","name":"Three Arrows Capital Portfolio","category":"PROPERTY"},{"slug":"polychain-capital-portfolio","name":"Polychain Capital Portfolio","category":"PROPERTY"},{"slug":"binance-labs-portfolio","name":"Binance Labs Portfolio","category":"PROPERTY"},{"slug":"blockchain-capital-portfolio","name":"Blockchain Capital Portfolio","category":"PROPERTY"},{"slug":"boostvc-portfolio","name":"BoostVC Portfolio","category":"PROPERTY"},{"slug":"cms-holdings-portfolio","name":"CMS Holdings Portfolio","category":"PROPERTY"},{"slug":"dcg-portfolio","name":"DCG Portfolio","category":"PROPERTY"},{"slug":"dragonfly-capital-portfolio","name":"DragonFly Capital Portfolio","category":"PROPERTY"},{"slug":"electric-capital-portfolio","name":"Electric Capital Portfolio","category":"PROPERTY"},{"slug":"fabric-ventures-portfolio","name":"Fabric Ventures Portfolio","category":"PROPERTY"},{"slug":"framework-ventures-portfolio","name":"Framework Ventures Portfolio","category":"PROPERTY"},{"slug":"galaxy-digital-portfolio","name":"Galaxy Digital Portfolio","category":"PROPERTY"},{"slug":"huobi-capital-portfolio","name":"Huobi Capital Portfolio","category":"PROPERTY"},{"slug":"alameda-research-portfolio","name":"Alameda Research Portfolio","category":"PROPERTY"},{"slug":"a16z-portfolio","name":"a16z Portfolio","category":"PROPERTY"},{"slug":"1confirmation-portfolio","name":"1Confirmation Portfolio","category":"PROPERTY"},{"slug":"winklevoss-capital-portfolio","name":"Winklevoss Capital Portfolio","category":"PROPERTY"},{"slug":"usv-portfolio","name":"USV Portfolio","category":"PROPERTY"},{"slug":"placeholder-ventures-portfolio","name":"Placeholder Ventures Portfolio","category":"PROPERTY"},{"slug":"pantera-capital-portfolio","name":"Pantera Capital Portfolio","category":"PROPERTY"},{"slug":"multicoin-capital-portfolio","name":"Multicoin Capital Portfolio","category":"PROPERTY"},{"slug":"paradigm-portfolio","name":"Paradigm Portfolio","category":"PROPERTY"}],"max_supply":21000000,"circulating_supply":19055662,"total_supply":19055662,"is_active":1,"platform":null,"cmc_rank":1,"is_fiat":0,"self_reported_circulating_supply":null,"self_reported_market_cap":null,"last_updated":"2022-06-01T21:30:00.000Z","quote":{"USD":{"price":29706.182790987863,"volume_24h":39796485079.81986,"volume_change_24h":10.0083,"percent_change_1h":-0.44330138,"percent_change_24h":-6.45915392,"percent_change_7d":-0.2142077,"percent_change_30d":-22.87949261,"percent_change_60d":-35.49696289,"percent_change_90d":-29.41247744,"market_cap":566070978575.2814,"market_cap_dominance":46.0595,"fully_diluted_market_cap":623829838610.75,"last_updated":"2022-06-01T21:30:00.000Z"}}}}}"#;
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

        assert_eq!(price, 29706.182790987863);
    }

    #[test]
    fn deserialize_struct_symbol() {
        let raw = r#"{"status":{"timestamp":"2022-06-01T19:16:46.519Z","error_code":0,"error_message":null,"elapsed":32,"credit_count":1,"notice":null},"data":{"BTC":[{"id":1,"name":"Bitcoin","symbol":"BTC","slug":"bitcoin","num_market_pairs":9474,"date_added":"2013-04-28T00:00:00.000Z","tags":[{"slug":"mineable","name":"Mineable","category":"OTHER"},{"slug":"pow","name":"PoW","category":"CONSENSUS_ALGORITHM"},{"slug":"sha-256","name":"SHA-256","category":"CONSENSUS_ALGORITHM"},{"slug":"store-of-value","name":"Store Of Value","category":"PROPERTY"},{"slug":"state-channel","name":"State Channel","category":"PROPERTY"},{"slug":"coinbase-ventures-portfolio","name":"Coinbase Ventures Portfolio","category":"PROPERTY"},{"slug":"three-arrows-capital-portfolio","name":"Three Arrows Capital Portfolio","category":"PROPERTY"},{"slug":"polychain-capital-portfolio","name":"Polychain Capital Portfolio","category":"PROPERTY"},{"slug":"binance-labs-portfolio","name":"Binance Labs Portfolio","category":"PROPERTY"},{"slug":"blockchain-capital-portfolio","name":"Blockchain Capital Portfolio","category":"PROPERTY"},{"slug":"boostvc-portfolio","name":"BoostVC Portfolio","category":"PROPERTY"},{"slug":"cms-holdings-portfolio","name":"CMS Holdings Portfolio","category":"PROPERTY"},{"slug":"dcg-portfolio","name":"DCG Portfolio","category":"PROPERTY"},{"slug":"dragonfly-capital-portfolio","name":"DragonFly Capital Portfolio","category":"PROPERTY"},{"slug":"electric-capital-portfolio","name":"Electric Capital Portfolio","category":"PROPERTY"},{"slug":"fabric-ventures-portfolio","name":"Fabric Ventures Portfolio","category":"PROPERTY"},{"slug":"framework-ventures-portfolio","name":"Framework Ventures Portfolio","category":"PROPERTY"},{"slug":"galaxy-digital-portfolio","name":"Galaxy Digital Portfolio","category":"PROPERTY"},{"slug":"huobi-capital-portfolio","name":"Huobi Capital Portfolio","category":"PROPERTY"},{"slug":"alameda-research-portfolio","name":"Alameda Research Portfolio","category":"PROPERTY"},{"slug":"a16z-portfolio","name":"a16z Portfolio","category":"PROPERTY"},{"slug":"1confirmation-portfolio","name":"1Confirmation Portfolio","category":"PROPERTY"},{"slug":"winklevoss-capital-portfolio","name":"Winklevoss Capital Portfolio","category":"PROPERTY"},{"slug":"usv-portfolio","name":"USV Portfolio","category":"PROPERTY"},{"slug":"placeholder-ventures-portfolio","name":"Placeholder Ventures Portfolio","category":"PROPERTY"},{"slug":"pantera-capital-portfolio","name":"Pantera Capital Portfolio","category":"PROPERTY"},{"slug":"multicoin-capital-portfolio","name":"Multicoin Capital Portfolio","category":"PROPERTY"},{"slug":"paradigm-portfolio","name":"Paradigm Portfolio","category":"PROPERTY"}],"max_supply":21000000,"circulating_supply":19055606,"total_supply":19055606,"is_active":1,"platform":null,"cmc_rank":1,"is_fiat":0,"self_reported_circulating_supply":null,"self_reported_market_cap":null,"last_updated":"2022-06-01T19:16:00.000Z","quote":{"USD":{"price":30181.22136320567,"volume_24h":36102185126.98055,"volume_change_24h":-9.5527,"percent_change_1h":-0.08911523,"percent_change_24h":-4.8113952,"percent_change_7d":1.10729351,"percent_change_30d":-21.29678875,"percent_change_60d":-34.32283146,"percent_change_90d":-28.78200486,"market_cap":575121462896.0302,"market_cap_dominance":46.2407,"fully_diluted_market_cap":633805648627.32,"last_updated":"2022-06-01T19:16:00.000Z"}}}]}}"#;
        let root: QuotesLatestV2Symbol = serde_json::from_str(raw).unwrap();
        let price = root.data.get("BTC").unwrap()[0]
            .quote
            .get("USD")
            .unwrap()
            .price;

        assert_eq!(price, 30181.22136320567);
    }
}

#[cfg(test)]
mod network_tests {

    #[test]
    fn net_price() {
        let apikey = option_env!("CMC_API").unwrap();
        use crate::Cmc;

        let cmc = Cmc::new(apikey);
        assert!(cmc.price("BTC").unwrap() > 0.1)
    }
}
