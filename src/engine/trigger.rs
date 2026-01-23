use crate::waha::models::WahaEvent;

#[derive(Debug, Clone)]
pub enum Trigger {
    Waha(WahaEvent),
    // Signal(SignalEvent), // Coming later
}
