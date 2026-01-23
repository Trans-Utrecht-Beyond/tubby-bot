use crate::engine::dispatcher::Handler;
use crate::engine::trigger::Trigger;
use async_trait::async_trait;
use log::info;

pub struct LoggingHandler;

#[async_trait]
impl Handler for LoggingHandler {
    async fn handle(&self, trigger: Trigger) {
        info!("Processing trigger: {:?}", trigger);
    }
}
