use dotenv::dotenv;
use serde::Deserialize;
use std::env;
#[derive(Debug, Deserialize)]
pub struct Config {
    pub openweather_api_key: String,
    pub weatherapi_api_key: String,
    pub influxdb_token: String,
    // pub influxdb_db_name: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv().ok();

        let openweather_api_key = env::var("OPENWEATHER_API_KEY")?;
        let weatherapi_api_key = env::var("WEATHERAPI_API_KEY")?;
        let influxdb_token = env::var("INFLUXDB_TOKEN")?;
        // let influxdb_db_name = env::var("INFLUXDB_DB_NAME")?;

        Ok(Config {
            openweather_api_key,
            weatherapi_api_key,
            influxdb_token,
            // influxdb_db_name,
        })
    }
}
