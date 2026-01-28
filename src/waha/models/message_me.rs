use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageMe {
    pub id: String,
    pub push_name: String,
    pub lid: Option<String>,
    pub jid: Option<String>,
}
