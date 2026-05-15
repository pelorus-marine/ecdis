//! Mapper traits and fusion helpers between Pelorus Core / Stream payloads and chart snapshots.

use std::time::{Duration, Instant};

use crate::{AisVesselReport, OwnShip};

/// Monotonic-friendly observation time carried alongside telemetry (nanoseconds since Unix epoch).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamInstant {
    pub unix_nanos: u128,
    pub source_label: Option<String>,
}

/// Own-ship snapshot plus optional Stream/Core observation time.
#[derive(Debug, Clone, PartialEq)]
pub struct TimedOwnShip {
    pub ship: OwnShip,
    pub observed_at: Option<StreamInstant>,
}

/// AIS report plus optional observation time.
#[derive(Debug, Clone, PartialEq)]
pub struct TimedAisTarget {
    pub report: AisVesselReport,
    pub observed_at: Option<StreamInstant>,
}

/// Implementations map opaque Core/Stream payloads into typed snapshots.
pub trait CoreSampleMapper: Send + Sync {
    fn map_own_ship(&self, payload: &[u8]) -> Option<TimedOwnShip>;
    fn map_ais(&self, payload: &[u8]) -> Option<TimedAisTarget>;
}

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

/// Prefer populated fields from `primary`, fill gaps from `secondary` (simple fusion stub).
pub fn merge_own_ship_fill_missing(primary: OwnShip, secondary: OwnShip) -> OwnShip {
    OwnShip {
        lat_deg: primary.lat_deg.or(secondary.lat_deg),
        lon_deg: primary.lon_deg.or(secondary.lon_deg),
        cog_true_deg: primary.cog_true_deg.or(secondary.cog_true_deg),
        sog_mps: primary.sog_mps.or(secondary.sog_mps),
        heading_true_deg: primary.heading_true_deg.or(secondary.heading_true_deg),
        depth_m: primary.depth_m.or(secondary.depth_m),
    }
}

/// Cheap monotonic clock anchor for correlating samples before GNSS time is available.
#[derive(Debug, Clone)]
pub struct FusionClock {
    origin: Instant,
}

impl Default for FusionClock {
    fn default() -> Self {
        Self::new()
    }
}

impl FusionClock {
    #[must_use]
    pub fn new() -> Self {
        Self {
            origin: Instant::now(),
        }
    }

    pub fn elapsed_monotonic(&self) -> Duration {
        self.origin.elapsed()
    }
}
