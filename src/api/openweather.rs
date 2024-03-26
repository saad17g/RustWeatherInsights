use crate::utils::http_client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct OpenWeatherResponse {
    main: MainData,
    name: String,
}

#[derive(Debug, Deserialize)]
struct MainData {
    temp: f64,
    humidity: f64,
}

pub async fn get_weather_data(
    api_key: &str,
    city: &str,
) -> Result<(String, f64, f64), reqwest::Error> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );
    let weather_data = http_client::get_json::<OpenWeatherResponse>(&url).await?;

    let city_name = weather_data.name;
    let temperature = weather_data.main.temp;
    let humidity = weather_data.main.humidity;

    Ok((city_name, temperature, humidity))
}
