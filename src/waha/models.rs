#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "lowercase")]
pub enum WahaEvent {
    Message(MessageEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageEvent {
    pub id: String,
    pub session: String,
    pub payload: MessagePayload,
    pub timestamp: u64,
    pub me: MessageMe,
    pub engine: String,
    pub environment: MessageEnvironment,
}

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
    pub media: Option<serde_json::Value>,
    pub ack: Option<i32>,
    pub ack_name: Option<String>,
    pub location: Option<serde_json::Value>,
    pub v_cards: Vec<serde_json::Value>,
    #[serde(rename = "_data")]
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageMe {
    pub id: String,
    pub push_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageEnvironment {
    pub version: String,
    pub engine: String,
    pub tier: String,
    pub browser: String,
    pub platform: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_message_event() {
        let json = r#"{
  "id": "evt_01kfmv5p0fme0jnsgbszxymrwm",
  "session": "default",
  "event": "message",
  "payload": {
    "id": "false_120363404882958348@g.us_3EB01C0FDFD037F51A4DEF_24610246492311@lid",
    "timestamp": 1769152370,
    "from": "120363404882958348@g.us",
    "fromMe": false,
    "participant": "24610246492311@lid",
    "source": "app",
    "to": "31612879047@c.us",
    "body": "Blooooo",
    "hasMedia": false,
    "media": null,
    "ack": 1,
    "ackName": "SERVER",
    "location": null,
    "vCards": [],
    "_data": {
      "id": {
        "fromMe": false,
        "remote": "120363404882958348@g.us",
        "id": "3EB01C0FDFD037F51A4DEF",
        "participant": "24610246492311@lid",
        "_serialized": "false_120363404882958348@g.us_3EB01C0FDFD037F51A4DEF_24610246492311@lid"
      }
    }
  },
  "timestamp": 1769152370703,
  "me": {
    "id": "31612879047@c.us",
    "pushName": "Trans Utrecht and Beyond"
  },
  "engine": "WEBJS",
  "environment": {
    "version": "2026.1.3",
    "engine": "WEBJS",
    "tier": "CORE",
    "browser": "/usr/bin/chromium",
    "platform": "linux/arm64"
  }
}"#;

        let event: WahaEvent = serde_json::from_str(json).unwrap();
        match event {
            WahaEvent::Message(msg) => {
                assert_eq!(msg.payload.body, "Blooooo");
                assert_eq!(msg.me.push_name, "Trans Utrecht and Beyond");
            }
        }
    }
}
