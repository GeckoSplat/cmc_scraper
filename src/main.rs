mod api;
mod args;
mod config;

use crate::args::Args;
use anyhow::{anyhow, Result};
use api::Response;
use clap::Parser;
use config::Config;
use serde_json::Value::Null;

fn main() -> Result<()> {
    let args = Args::parse();

    println!(
        "\nArguments are {},{},{}\n",
        args.start, args.limit, args.convert
    );
    let arg_to_print = args.convert.clone();

    let text = std::fs::read_to_string("config.json").expect("config ERROR");
    let config = serde_json::from_str::<Config>(&text).expect("config serde ERROR");

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
        .expect("Response ERROR")
        .text()
        .expect("Response ERROR");

    let result: Response = serde_json::from_str(&response).unwrap();

    if result.status.error_message != Null {
        return Err(anyhow!("{}", "API STATUS ERROR"));
    }
    for crypto in result.data {
        println!(
            "Name: {}\nSymbol: {}\nPrice in {arg_to_print}:  {}\n24h % Change: {}\n",
            crypto.name,
            crypto.symbol,
            crypto
                .quote
                .get(&(arg_to_print.clone()))
                .expect("quote ERROR")
                .price,
            crypto
                .quote
                .get(&(arg_to_print.clone()))
                .expect("denomination arg ERROR")
                .percent_change_24h
        );
    }

    Ok(())
}
