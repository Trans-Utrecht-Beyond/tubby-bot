use crate::config::Config;
use crate::engine::dispatcher::Handler;
use crate::engine::trigger::Trigger;
use crate::signal::actions as signal_actions;
use crate::signal::models::{SendMessageRequest, TypingIndicatorRequest};
use crate::utils::get_plausible_typing_time;
use crate::waha::actions as waha_actions;
use crate::waha::models::WahaEvent;
use base64::{Engine as _, engine::general_purpose};
use log::debug;
use std::sync::Arc;

pub struct ForwardAnnouncementsHandler {
    config: Arc<Config>,
}

impl ForwardAnnouncementsHandler {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl Handler for ForwardAnnouncementsHandler {
    fn condition(&self, trigger: &Trigger) -> bool {
        if let Trigger::Waha(WahaEvent::Message(event)) = trigger {
            if event.payload.from_me {
                return false;
            }
            if event.payload.from
                == self
                    .config
                    .forward_announcements_config
                    .whatsapp_source_chat_id
            {
                return true;
            }
        }
        false
    }

    async fn handle(&self, trigger: Trigger) {
        if let Trigger::Waha(WahaEvent::Message(message_event)) = trigger {
            debug!("Forwarding message to signal: {}", message_event.payload.id);
            let typing_time = get_plausible_typing_time(message_event.payload.body.clone());

            let base64_attachments = if let Some(media) = &message_event.payload.media {
                waha_actions::download_media(&media.url, &self.config.waha_api_key)
                    .await
                    .ok()
                    .map(|bytes| vec![general_purpose::STANDARD.encode(bytes)])
            } else {
                None
            };

            signal_actions::type_for_ms(
                &self.config.signal_base_url,
                TypingIndicatorRequest {
                    recipient: self
                        .config
                        .forward_announcements_config
                        .signal_destination_chat_id
                        .clone(),
                },
                typing_time,
            )
            .await
            .unwrap();

            let request = SendMessageRequest {
                base64_attachments,
                ..SendMessageRequest::new(
                    message_event.payload.body.clone(),
                    vec![
                        self.config
                            .forward_announcements_config
                            .signal_destination_chat_id
                            .clone(),
                    ],
                )
            };

            signal_actions::send_message(&self.config.signal_base_url, request)
                .await
                .unwrap();
        }
    }
}
