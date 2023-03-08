mod api;
mod config;

use anyhow::Result;
use api::{Response, Status};
use clap::Parser;
use config::Config;
use serde_json::Value::Null;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    start: String,

    #[arg(short, long)]
    limit: String,

    #[arg(short, long)]
    convert: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!(
        "\nArguments are {},{},{}\n",
        args.start, args.limit, args.convert
    );
    let arg_to_print = args.convert.clone();
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

    if result.status.error_message != Null {
        println!("API STATUS ERROR !")
    } else {
        for crypto in result.data {
            println!(
                "Name: {}\nSymbol: {}\nPrice in {arg_to_print}:  {}\n24h % Change: {}\n",
                crypto.name,
                crypto.symbol,
                crypto.quote.get(&String::from(arg_to_print.clone())).unwrap().price,
                crypto.quote.get(&String::from(arg_to_print.clone())).unwrap().percent_change_24h
            );
        }
    }

    Ok(())
}
