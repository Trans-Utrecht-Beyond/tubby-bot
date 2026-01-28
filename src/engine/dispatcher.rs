use crate::engine::trigger::Trigger;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::task::JoinSet;

#[async_trait]
pub trait Handler: Send + Sync {
    fn condition(&self, trigger: &Trigger) -> bool;
    async fn handle(&self, trigger: Trigger);
}

pub struct Dispatcher {
    handlers: Vec<Arc<dyn Handler>>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn register_handler(&mut self, handler: Arc<dyn Handler>) {
        self.handlers.push(handler);
    }

    pub async fn dispatch(&self, trigger: Trigger) {
        let mut set = JoinSet::new();

        for handler in &self.handlers {
            if !handler.condition(&trigger) {
                continue;
            }

            let handler_clone = handler.clone();
            let trigger_clone = trigger.clone();

            set.spawn(async move {
                handler_clone.handle(trigger_clone).await;
            });
        }

        // Wait for all handlers to complete
        while let Some(_) = set.join_next().await {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    struct TestHandler {
        name: String,
        executed: Arc<Mutex<Vec<String>>>,
    }

    #[async_trait]
    impl Handler for TestHandler {
        fn condition(&self, _trigger: &Trigger) -> bool {
            true
        }

        async fn handle(&self, _trigger: Trigger) {
            let mut executed = self.executed.lock().unwrap();
            executed.push(self.name.clone());
        }
    }

    #[tokio::test]
    async fn test_dispatcher_parallel_execution() {
        let executed = Arc::new(Mutex::new(Vec::new()));
        let mut dispatcher = Dispatcher::new();

        dispatcher.register_handler(Arc::new(TestHandler {
            name: "handler1".to_string(),
            executed: executed.clone(),
        }));
        dispatcher.register_handler(Arc::new(TestHandler {
            name: "handler2".to_string(),
            executed: executed.clone(),
        }));

        let trigger = Trigger::Waha(crate::waha::models::WahaEvent::Message(
            crate::waha::models::MessageEvent {
                id: "1".to_string(),
                session: "default".to_string(),
                payload: crate::waha::models::MessagePayload {
                    id: "1".to_string(),
                    timestamp: 1,
                    from: "1".to_string(),
                    from_me: false,
                    participant: None,
                    source: None,
                    to: "1".to_string(),
                    body: "payload".to_string(),
                    has_media: false,
                    media: None,
                    ack: None,
                    ack_name: None,
                    location: None,
                    v_cards: None,
                    reply_to: None,
                    data: serde_json::json!({}),
                },
                timestamp: 1,
                me: crate::waha::models::MessageMe {
                    id: "1".to_string(),
                    push_name: "1".to_string(),
                    lid: None,
                    jid: None,
                },
                engine: Some("1".to_string()),
                environment: crate::waha::models::MessageEnvironment {
                    version: "1".to_string(),
                    engine: "1".to_string(),
                    tier: "1".to_string(),
                    browser: Some("1".to_string()),
                    platform: "1".to_string(),
                    worker: None,
                },
            },
        ));

        dispatcher.dispatch(trigger).await;

        let executed_names = executed.lock().unwrap();
        assert_eq!(executed_names.len(), 2);
        assert!(executed_names.contains(&"handler1".to_string()));
        assert!(executed_names.contains(&"handler2".to_string()));
    }

    struct ConditionalHandler {
        should_run: bool,
        executed: Arc<Mutex<bool>>,
    }

    #[async_trait]
    impl Handler for ConditionalHandler {
        fn condition(&self, _trigger: &Trigger) -> bool {
            self.should_run
        }

        async fn handle(&self, _trigger: Trigger) {
            let mut executed = self.executed.lock().unwrap();
            *executed = true;
        }
    }

    #[tokio::test]
    async fn test_dispatcher_respects_condition() {
        let executed_true = Arc::new(Mutex::new(false));
        let executed_false = Arc::new(Mutex::new(false));
        let mut dispatcher = Dispatcher::new();

        dispatcher.register_handler(Arc::new(ConditionalHandler {
            should_run: true,
            executed: executed_true.clone(),
        }));
        dispatcher.register_handler(Arc::new(ConditionalHandler {
            should_run: false,
            executed: executed_false.clone(),
        }));

        let trigger = Trigger::Waha(crate::waha::models::WahaEvent::Message(
            crate::waha::models::MessageEvent {
                id: "1".to_string(),
                session: "default".to_string(),
                payload: crate::waha::models::MessagePayload {
                    id: "1".to_string(),
                    timestamp: 1,
                    from: "1".to_string(),
                    from_me: false,
                    participant: None,
                    source: None,
                    to: "1".to_string(),
                    body: "payload".to_string(),
                    has_media: false,
                    media: None,
                    ack: None,
                    ack_name: None,
                    location: None,
                    v_cards: None,
                    reply_to: None,
                    data: serde_json::json!({}),
                },
                timestamp: 1,
                me: crate::waha::models::MessageMe {
                    id: "1".to_string(),
                    push_name: "1".to_string(),
                    lid: None,
                    jid: None,
                },
                engine: Some("1".to_string()),
                environment: crate::waha::models::MessageEnvironment {
                    version: "1".to_string(),
                    engine: "1".to_string(),
                    tier: "1".to_string(),
                    browser: Some("1".to_string()),
                    platform: "1".to_string(),
                    worker: None,
                },
            },
        ));

        dispatcher.dispatch(trigger).await;

        assert!(*executed_true.lock().unwrap());
        assert!(!*executed_false.lock().unwrap());
    }
}
