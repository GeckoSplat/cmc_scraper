# Rust project to get crypto prices from the coinmarketcap.com API.

1. Go to [coinmarketcap](https://coinmarketcap.com/api/) and create an API key.
2. Copy the file `config.json.ex` and name it `config.json`
3. In `config.json` enter your API key
4. Run the executable with `cargo run` with the CLI arguments as follows  "--start **integer** --limit **integer** --convert **USD,GBP** etc(only one) "

Results are displayed in the terminal.
