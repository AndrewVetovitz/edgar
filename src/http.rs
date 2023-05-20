use crate::EdgarAPI;
use std::error::Error;

use reqwest::Response;

pub async fn get(wrapper: &EdgarAPI, endpoint: &str) -> Result<Response, Box<dyn Error>> {
    Ok(wrapper.http_client.get(endpoint).send().await?)
}