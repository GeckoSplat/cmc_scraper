use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub apikey: String,
}
