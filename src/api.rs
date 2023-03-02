use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

// Took some inspiration from
// https://github.com/Dzhegash/cmc/blob/master/src/api/cryptocurrency/quotes_latest_v2.rs

#[derive(Deserialize, Debug)]
pub struct Response {
    pub data: Vec<Coin>,
    pub status: Status,
}

#[derive(Deserialize, Debug)]
pub struct Status {
    pub timestamp: String,
    pub error_code: u64,
    pub error_message: Value,
    pub elapsed: u64,
    pub credit_count: u64,
}

#[derive(Deserialize, Debug)]
pub struct Coin {
    pub id: u64,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    pub cmc_rank: u64,
    pub num_market_pairs: u64,
    pub circulating_supply: Value,
    pub total_supply: Value,
    pub max_supply: Value,
    pub last_updated: String,
    pub date_added: String,
    pub tags: Vec<String>,
    pub platform: Value,
    pub self_reported_circulating_supply: Value,
    pub self_reported_market_cap: Value,
    pub tvl_ratio: Value,
    pub quote: HashMap<String, Currency>,
}

#[derive(Deserialize, Debug)]
pub struct Currency {
    pub price: f64,
    pub volume_24h: f64,
    pub volume_change_24h: f64,
    pub percent_change_1h: f64,
    pub percent_change_24h: f64,
    pub percent_change_7d: f64,
    pub market_cap: f64,
    pub market_cap_dominance: f64,
    pub fully_diluted_market_cap: f64,
    pub last_updated: String,
}
