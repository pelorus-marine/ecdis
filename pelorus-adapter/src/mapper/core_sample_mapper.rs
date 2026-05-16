use super::{TimedAisTarget, TimedOwnShip};

/// Implementations map opaque Core/Stream payloads into typed snapshots.
pub trait CoreSampleMapper: Send + Sync {
    fn map_own_ship(&self, payload: &[u8]) -> Option<TimedOwnShip>;
    fn map_ais(&self, payload: &[u8]) -> Option<TimedAisTarget>;
}
