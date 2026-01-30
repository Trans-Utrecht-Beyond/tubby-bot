use crate::config::Config;
use crate::engine::dispatcher::Handler;
use crate::engine::trigger::Trigger;
use crate::signal::actions as signal_actions;
use crate::signal::models::{SendMessageRequest, TypingIndicatorRequest};
use crate::utils::get_plausible_typing_time;
use crate::waha::models::WahaEvent;
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

            signal_actions::send_message(
                &self.config.signal_base_url,
                SendMessageRequest::new(
                    message_event.payload.body.clone(),
                    vec![
                        self.config
                            .forward_announcements_config
                            .signal_destination_chat_id
                            .clone(),
                    ],
                ),
            )
            .await
            .unwrap();
        }
    }
}
