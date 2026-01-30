use crate::config::Config;
use crate::engine::dispatcher::Handler;
use crate::engine::trigger::Trigger;
use crate::waha::actions::send_seen;
use crate::waha::models::{SendSeenRequest, WahaEvent};
use async_trait::async_trait;
use log::{debug, error};
use std::sync::Arc;

pub struct WahaSendSeenHandler {
    pub config: Arc<Config>,
}

impl WahaSendSeenHandler {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

#[async_trait]
impl Handler for WahaSendSeenHandler {
    fn condition(&self, trigger: &Trigger) -> bool {
        matches!(trigger, Trigger::Waha(WahaEvent::Message(m)) if !m.payload.from_me)
    }

    async fn handle(&self, trigger: Trigger) {
        if let Trigger::Waha(WahaEvent::Message(message_event)) = trigger {
            debug!("Sending seen for message: {}", message_event.payload.id);
            let send_seen_request = SendSeenRequest::new(
                message_event.payload.id.clone(),
                message_event.session.clone(),
                vec![message_event.payload.id.clone()],
            );
            if let Err(e) = send_seen(
                &self.config.waha_base_url,
                &self.config.waha_api_key,
                send_seen_request,
            )
            .await
            {
                error!("Failed to send seen: {}", e);
            }
        }
    }
}
