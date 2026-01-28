use super::{MessageEnvironment, MessageMe, MessagePayload};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageEvent {
    pub id: String,
    pub session: String,
    pub payload: MessagePayload,
    pub timestamp: u64,
    pub me: MessageMe,
    pub engine: Option<String>,
    pub environment: MessageEnvironment,
}
