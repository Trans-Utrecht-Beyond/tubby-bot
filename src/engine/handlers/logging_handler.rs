use crate::config::Config;
use crate::engine::dispatcher::Handler;
use crate::engine::trigger::Trigger;
use async_trait::async_trait;
use log::info;
use std::sync::Arc;

pub struct LoggingHandler {
    #[allow(unused)]
    pub config: Arc<Config>,
}

#[async_trait]
impl Handler for LoggingHandler {
    fn condition(&self, _trigger: &Trigger) -> bool {
        true
    }

    async fn handle(&self, trigger: Trigger) {
        info!("Processing trigger: {:?}", trigger);
    }
}
