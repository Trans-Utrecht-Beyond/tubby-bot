use crate::config::Config;
use crate::engine::dispatcher::Handler;
use crate::engine::trigger::Trigger;
use async_trait::async_trait;
use log::debug;
use std::sync::Arc;

pub struct LoggingHandler {
    #[allow(unused)]
    pub config: Arc<Config>,
}

impl LoggingHandler {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

#[async_trait]
impl Handler for LoggingHandler {
    fn condition(&self, _trigger: &Trigger) -> bool {
        true
    }

    async fn handle(&self, trigger: Trigger) {
        debug!("Processing trigger: {:?}", trigger);
    }
}
