mod api;
mod config;

use api::Response;
use config::Config;

fn main() {
    // Add error handling for I/O errors
    let text = std::fs::read_to_string("config.json").unwrap();
    let config = serde_json::from_str::<Config>(&text).unwrap();

    let cm_url = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest".to_string();
    let client = reqwest::blocking::Client::new();

    // TODO: make query parameters configurable
    let response = client
        .get(cm_url)
        .header("X-CMC_PRO_API_KEY", config.apikey)
        .query(&[("start", "1"), ("limit", "5"), ("convert", "GBP")])
        .send()
        .unwrap()
        .text()
        .unwrap();

    let result: Response = serde_json::from_str(&response).unwrap();

    // TODO: can check status to see if error, inform user of error or print result if ok
    for crypto in result.data {
        // TODO: make currency configurable
        println!(
            "Name: {}\nSymbol: {}\nPrice £{}\n24h % Change: {}\n",
            crypto.name,
            crypto.symbol,
            crypto.quote.get("GBP").unwrap().price,
            crypto.quote.get("GBP").unwrap().percent_change_24h
        );
    }
}