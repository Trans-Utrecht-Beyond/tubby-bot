use crate::signal::models::{SendMessageRequest, SendMessageResponse, TypingIndicatorRequest};
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

pub async fn send_typing_indicator(
    base_url: &str,
    request: TypingIndicatorRequest,
) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = format!("{}/v1/typing-indicator", base_url);
    client.put(url).json(&request).send().await?;

    Ok(())
}

pub async fn stop_typing_indicator(
    base_url: &str,
    request: TypingIndicatorRequest,
) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = format!("{}/v1/typing-indicator", base_url);
    client.delete(url).json(&request).send().await?;

    Ok(())
}

pub async fn type_for_ms(
    base_url: &str,
    request: TypingIndicatorRequest,
    ms: u64,
) -> Result<(), Box<dyn Error>> {
    send_typing_indicator(base_url, request.clone()).await?;
    tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
    stop_typing_indicator(base_url, request).await?;

    Ok(())
}
