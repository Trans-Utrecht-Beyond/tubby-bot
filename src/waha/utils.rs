pub fn get_headers(api_key: &str) -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "X-Api-Key",
        reqwest::header::HeaderValue::from_str(api_key).unwrap(),
    );
    headers
}
