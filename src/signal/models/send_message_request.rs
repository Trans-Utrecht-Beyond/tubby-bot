use super::{LinkPreview, Mention};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub message: String,
    pub number: String,
    pub recipients: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base64_attachments: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview: Option<LinkPreview>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<Mention>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_self: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_mentions: Option<Vec<Mention>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_once: Option<bool>,
}

impl SendMessageRequest {
    pub fn new(message: String, number: String, recipients: Vec<String>) -> Self {
        Self {
            message,
            number,
            recipients,
            base64_attachments: None,
            edit_timestamp: None,
            link_preview: None,
            mentions: None,
            notify_self: None,
            quote_author: None,
            quote_mentions: None,
            quote_message: None,
            quote_timestamp: None,
            sticker: None,
            text_mode: None,
            view_once: None,
        }
    }
}
