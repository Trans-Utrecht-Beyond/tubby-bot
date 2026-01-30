pub mod message_environment;
pub mod message_event;
pub mod message_me;
pub mod message_payload;
pub mod send_seen_request;
pub mod waha_event;

pub use message_environment::MessageEnvironment;
pub use message_event::MessageEvent;
pub use message_me::MessageMe;
pub use message_payload::MessagePayload;
pub use send_seen_request::SendSeenRequest;
pub use waha_event::WahaEvent;

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
        let WahaEvent::Message(msg) = event;
        assert_eq!(msg.payload.body, "Blooooo");
        assert_eq!(msg.me.push_name, "Trans Utrecht and Beyond");
    }

    #[test]
    fn test_deserialize_gow_message_event() {
        let json = r#"{
  "id": "evt_01kg0z18webgjjc8jyk0mvg07b",
  "session": "default",
  "event": "message",
  "payload": {
    "id": "false_120363408101232619@g.us_AC79C12F3AD4F6D4A008CF6F141130D5_24610246492311@lid",
    "timestamp": 1769559073,
    "from": "120363408101232619@g.us",
    "fromMe": false,
    "source": "app",
    "body": "Bla",
    "to": "24610246492311@lid",
    "participant": "24610246492311@lid",
    "hasMedia": false,
    "media": null,
    "ack": 2,
    "location": null,
    "vCards": null,
    "ackName": "DEVICE",
    "replyTo": null,
    "_data": {}
  },
  "timestamp": 1769559073678,
  "me": {
    "id": "31612879047@c.us",
    "pushName": "Trans Utrecht and Beyond",
    "lid": "69003800264704@lid",
    "jid": "31612879047:2@s.whatsapp.net"
  },
  "environment": {
    "version": "2026.1.4",
    "engine": "GOWS",
    "tier": "CORE",
    "browser": null,
    "platform": "linux/arm64",
    "worker": {
      "id": null
    }
  }
}"#;

        let event: WahaEvent =
            serde_json::from_str(json).expect("Failed to deserialize GOWS event");
        let WahaEvent::Message(msg) = event;
        assert_eq!(msg.payload.body, "Bla");
        assert!(msg.engine.is_none());
        assert_eq!(msg.environment.engine, "GOWS");
    }
}
