use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mention {
    pub author: String,
    pub start: u32,
    pub length: u32,
}
