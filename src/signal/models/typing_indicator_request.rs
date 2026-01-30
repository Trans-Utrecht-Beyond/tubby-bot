use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TypingIndicatorRequest {
    pub recipient: String,
}
