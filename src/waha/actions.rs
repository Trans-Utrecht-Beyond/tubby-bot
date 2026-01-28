use crate::waha::{models::SendSeenRequest, utils};
use std::error::Error;

pub async fn send_seen(
    base_url: &str,
    api_key: &str,
    request: SendSeenRequest,
) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = format!("{}/api/sendSeen", base_url);
    client
        .post(url)
        .json(&request)
        .headers(utils::get_headers(api_key))
        .send()
        .await?;

    Ok(())
}
