use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagePayload {
    pub id: String,
    pub timestamp: u64,
    pub from: String,
    pub from_me: bool,
    pub participant: Option<String>,
    pub source: Option<String>,
    pub to: String,
    pub body: String,
    pub has_media: bool,
    pub media: Option<super::WahaMedia>,
    pub ack: Option<i32>,
    pub ack_name: Option<String>,
    pub location: Option<serde_json::Value>,
    pub v_cards: Option<Vec<serde_json::Value>>,
    pub reply_to: Option<serde_json::Value>,
    #[serde(rename = "_data")]
    pub data: serde_json::Value,
}
