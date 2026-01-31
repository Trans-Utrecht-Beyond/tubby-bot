use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WahaMedia {
    pub url: String,
    pub mimetype: String,
    pub filename: Option<String>,
    pub s3: Option<WahaMediaS3>,
    pub error: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WahaMediaS3 {
    pub bucket: String,
    pub key: String,
}
