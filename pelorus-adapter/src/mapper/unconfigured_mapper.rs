use super::{CoreSampleMapper, TimedAisTarget, TimedOwnShip};

/// Placeholder mapper — returns [`None`] until DCID bindings are implemented upstream.
#[derive(Debug, Default, Clone, Copy)]
pub struct UnconfiguredMapper;

impl CoreSampleMapper for UnconfiguredMapper {
    fn map_own_ship(&self, _payload: &[u8]) -> Option<TimedOwnShip> {
        None
    }

    fn map_ais(&self, _payload: &[u8]) -> Option<TimedAisTarget> {
        None
    }
}
