use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendSeenRequest {
    pub chat_id: String,
    pub message_ids: Option<Vec<String>>,
    pub participant: Option<String>,
    pub session: String,
}

impl SendSeenRequest {
    pub fn new(chat_id: String, session: String, message_ids: Vec<String>) -> Self {
        Self {
            chat_id,
            message_ids: Some(message_ids),
            participant: None,
            session,
        }
    }
}
