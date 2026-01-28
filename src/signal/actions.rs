use crate::signal::models::{SendMessageRequest, SendMessageResponse};
use std::error::Error;

pub async fn send_message(
    base_url: &str,
    request: SendMessageRequest,
) -> Result<SendMessageResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = format!("{}/v2/send", base_url);
    let response = client.post(url).json(&request).send().await?;

    let response = response.json::<SendMessageResponse>().await?;
    Ok(response)
}
