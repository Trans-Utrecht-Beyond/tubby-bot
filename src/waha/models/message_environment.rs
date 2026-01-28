use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageEnvironment {
    pub version: String,
    pub engine: String,
    pub tier: String,
    pub browser: Option<String>,
    pub platform: String,
    pub worker: Option<serde_json::Value>,
}
