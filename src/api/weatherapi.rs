use crate::utils::http_client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct WeatherApiResponse {
    location: Location,
    current: CurrentWeather,
}

#[derive(Debug, Deserialize)]
struct Location {
    name: String,
}

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    temp_c: f64,
    humidity: f64,
}

pub async fn get_weather_data(
    api_key: &str,
    city: &str,
) -> Result<(String, f64, f64), reqwest::Error> {
    let url = format!(
        "http://api.weatherapi.com/v1/current.json?key={}&q={}",
        api_key, city
    );
    let weather_data = http_client::get_json::<WeatherApiResponse>(&url).await?;

    let city_name = weather_data.location.name;
    let temperature = weather_data.current.temp_c;
    let humidity = weather_data.current.humidity;

    Ok((city_name, temperature, humidity))
}
