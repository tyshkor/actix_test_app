use dotenv::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
}

impl Config {

    pub fn from_env() -> envy::Result<Config> {
        dotenv().ok();

        envy::from_env::<Config>()
    }
}