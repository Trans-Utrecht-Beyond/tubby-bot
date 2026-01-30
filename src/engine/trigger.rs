use crate::waha::models::WahaEvent;

#[derive(Debug, Clone)]
pub enum Trigger {
    Waha(WahaEvent),
    FakeTriggerToStopErrors,
    // Signal(SignalEvent), // Coming later
}
