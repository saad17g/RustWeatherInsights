use chrono::{DateTime, Utc};
use influxdb::{Client, Error, InfluxDbWriteable, ReadQuery};

pub async fn write_data(
    url: &str,
    temp: f64,
    hum: f64,
    city: &str,
    token: &str,
    bucket: &str,
) -> Result<(), Error> {
    // Connect to db `test` on `http://localhost:8086`
    let client = Client::new(url, bucket).with_token(token);

    #[derive(InfluxDbWriteable)]
    struct WeatherReading {
        time: DateTime<Utc>,
        humidity: f64,
        temperature: f64,
        #[influxdb(tag)]
        city: String,
    }

    // Let's write some data into a measurement called `weather`
    let weather_readings = vec![WeatherReading {
        time: Utc::now(), // Use the current time
        humidity: hum,
        temperature: temp,
        city: city.to_string(),
    }
    .into_query("weather_data")];

    client.query(weather_readings).await?;

    // Read back all records
    let read_query = ReadQuery::new("SELECT * FROM weather_data");

    let read_result = client.query(read_query).await?;
    println!("wrote data: {} to influxdb", read_result);
    Ok(())
}
