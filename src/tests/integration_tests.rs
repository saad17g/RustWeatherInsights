use crate::api::openweather;
use crate::api::weatherapi;
use crate::config::Config;

#[tokio::test]
async fn test_openweathermap_api() {
    let config = Config::from_env().expect("Failed to load configuration");
    let city = "London";

    match openweather::get_weather_data(&config.openweather_api_key, city).await {
        Ok((city_name, temperature, humidity)) => {
            assert_eq!(city_name, "London");
            assert!(temperature > -50.0 && temperature < 50.0);
            assert!(humidity >= 0.0 && humidity <= 100.0);
        }
        Err(e) => {
            panic!("Error: {}", e);
        }
    }
}

#[tokio::test]
async fn test_weatherapi_api() {
    let config = Config::from_env().expect("Failed to load configuration");
    let city = "Paris";

    match weatherapi::get_weather_data(&config.weatherapi_api_key, city).await {
        Ok((city_name, temperature, humidity)) => {
            assert_eq!(city_name, "Paris");
            assert!(temperature > -50.0 && temperature < 50.0);
            assert!(humidity >= 0.0 && humidity <= 100.0);
        }
        Err(e) => {
            panic!("Error: {}", e);
        }
    }
}
