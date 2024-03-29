use influxdb::Error;
use rust_weather_insights::api::{openweather, weatherapi};
use rust_weather_insights::config::Config;
use rust_weather_insights::data_processing::arrow;
use rust_weather_insights::storage::influx_db;

const CITY: &str = "Paris";
const BUCKET: &str = "b1";

#[tokio::main]
async fn main() -> Result<(), Error> {
    // call the 2 APIs
    let config = Config::from_env().expect("Failed to load configuration");

    let openweather_result = openweather::get_weather_data(&config.openweather_api_key, CITY)
        .await
        .unwrap();
    let weatherapi_result = weatherapi::get_weather_data(&config.weatherapi_api_key, CITY)
        .await
        .unwrap();

    let data: Vec<(f64, f64)> = vec![openweather_result, weatherapi_result]
        .into_iter()
        .map(|(_, x, y)| (x, y))
        .collect();

    let processed_data = arrow::process_weather_data(data).unwrap();

    let (avg_temp, avg_hum) = processed_data;

    // calculate the average
    influx_db::write_data(
        "http://localhost:8086",
        avg_temp,
        avg_hum,
        CITY,
        &config.influxdb_token,
        BUCKET,
    )
    .await
}
