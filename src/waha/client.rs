use crate::config::Config;
use crate::engine::dispatcher::Dispatcher;
use crate::engine::trigger::Trigger;
use crate::waha::models::WahaEvent;
use futures_util::StreamExt;
use log::{error, info, warn};
use std::sync::Arc;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

pub struct WahaClient {
    config: Config,
    dispatcher: Arc<Dispatcher>,
}

impl WahaClient {
    pub fn new(config: Config, dispatcher: Arc<Dispatcher>) -> Self {
        Self { config, dispatcher }
    }

    pub fn get_ws_url(&self) -> String {
        let ws_base_url = if self.config.base_url.starts_with("https://") {
            self.config.base_url.replace("https://", "wss://")
        } else {
            self.config.base_url.replace("http://", "ws://")
        };

        format!(
            "{}/ws?session=*&events=*&x-api-key={}",
            ws_base_url, self.config.api_key
        )
    }

    pub async fn listen(&self) -> Result<(), Box<dyn std::error::Error>> {
        let ws_url = self.get_ws_url();
        info!(
            "Connecting to WAHA at: {}",
            ws_url.split('?').next().unwrap_or(&ws_url)
        );

        let (ws_stream, _) = connect_async(ws_url).await?;
        info!("WebSocket connection established");

        let (_, mut read) = ws_stream.split();

        while let Some(message) = read.next().await {
            match message {
                Ok(msg) => {
                    if !self.handle_message(msg).await {
                        break;
                    }
                }
                Err(e) => {
                    error!("Error receiving message: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    async fn handle_message(&self, msg: Message) -> bool {
        match msg {
            Message::Text(text) => {
                self.handle_event_text(&text).await;
                true
            }
            Message::Close(frame) => {
                warn!("Connection closed: {:?}", frame);
                false
            }
            msg => {
                warn!("Received unhandled message: {:?}", msg);
                true
            }
        }
    }

    async fn handle_event_text(&self, text: &str) {
        match serde_json::from_str::<WahaEvent>(text) {
            Ok(event) => {
                self.handle_waha_event(event).await;
            }
            Err(_) => match serde_json::from_str::<serde_json::Value>(text) {
                Ok(json) => {
                    info!(
                        "Received unknown event: {}",
                        json.get("event")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown")
                    );
                }
                Err(e) => {
                    warn!("Received raw text (failed to parse JSON: {}): {}", e, text);
                }
            },
        }
    }

    async fn handle_waha_event(&self, event: WahaEvent) {
        self.dispatcher.dispatch(Trigger::Waha(event)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_client() -> WahaClient {
        let config = Config {
            api_key: "key".to_string(),
            base_url: "http://localhost:3000".to_string(),
        };
        let dispatcher = Arc::new(Dispatcher::new());
        WahaClient::new(config, dispatcher)
    }

    #[test]
    fn test_get_ws_url() {
        let client = setup_client();
        let url = client.get_ws_url();
        assert!(url.starts_with("ws://localhost:3000/ws"));
        assert!(url.contains("x-api-key=key"));

        let config_tls = Config {
            api_key: "key".to_string(),
            base_url: "https://example.com".to_string(),
        };
        let dispatcher = Arc::new(Dispatcher::new());
        let client_tls = WahaClient::new(config_tls, dispatcher);
        assert!(client_tls.get_ws_url().starts_with("wss://example.com/ws"));
    }

    #[tokio::test]
    async fn test_handle_event_text_valid_json() {
        let client = setup_client();
        client.handle_event_text(r#"{"event": "unknown"}"#).await;
    }

    #[tokio::test]
    async fn test_handle_event_text_message_event() {
        let client = setup_client();
        let json = r#"{
            "id": "evt_1",
            "session": "default",
            "event": "message",
            "payload": {
                "id": "msg_1",
                "timestamp": 12345,
                "from": "user_1",
                "fromMe": false,
                "to": "me",
                "body": "Hi",
                "hasMedia": false,
                "vCards": [],
                "_data": {}
            },
            "timestamp": 123456,
            "me": { "id": "me", "pushName": "Bot" },
            "engine": "WEBJS",
            "environment": {
                "version": "1.0",
                "engine": "WEBJS",
                "tier": "CORE",
                "browser": "chrome",
                "platform": "linux"
            }
        }"#;
        client.handle_event_text(json).await;
    }

    #[tokio::test]
    async fn test_handle_event_text_invalid_json() {
        let client = setup_client();
        client.handle_event_text("not json").await;
    }
}
