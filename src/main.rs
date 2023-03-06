mod api;
mod config;

use clap::Parser;
//std::env::args;
use anyhow::Result;
use api::{Response, Status};
use config::Config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    start: String,

    #[arg(short, long)]
    limit: String,

    #[arg(short, long)]
    convert: String,
}

fn main() {
    // Add error handling for I/O errors
    // cannot figure out try catch in rust

    let args = Args::parse();

    println!("{},{},{}", args.start, args.limit, args.convert);

    let text = std::fs::read_to_string("config.json").unwrap();
    let config = serde_json::from_str::<Config>(&text).unwrap();

    let cm_url = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest".to_string();
    let client = reqwest::blocking::Client::new();

    let response = client
        .get(cm_url)
        .header("X-CMC_PRO_API_KEY", config.apikey)
        .query(&[
            ("start", args.start),
            ("limit", args.limit),
            ("convert", args.convert),
        ])
        .send()
        .unwrap()
        .text()
        .unwrap();

    let result: Response = serde_json::from_str(&response).unwrap();

    // TODO: can check status to see if error, inform user of error or print result if ok

    for crypto in result.data {
        // TODO: make currency configurable
        // need to change url and api.rs to manage this
        println!(
            "Name: {}\nSymbol: {}\nPrice £{}\n24h % Change: {}\n",
            crypto.name,
            crypto.symbol,
            crypto.quote.get("GBP").unwrap().price,
            crypto.quote.get("GBP").unwrap().percent_change_24h
        );
    }
}
