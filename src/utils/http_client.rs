use reqwest::{Client, Error, Response};
use serde::de::DeserializeOwned;

// Sends an asynchronous GET request to the specified URL and returns the response.
pub async fn send_request(url: &str) -> Result<Response, Error> {
    let client = Client::new();
    client.get(url).send().await
}

// Sends a GET request to the specified URL and deserializes the JSON response.
pub async fn get_json<T: DeserializeOwned>(url: &str) -> Result<T, Error> {
    let response = send_request(url).await?;
    response.json::<T>().await
}
