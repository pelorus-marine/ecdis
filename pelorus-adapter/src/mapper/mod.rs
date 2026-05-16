//! Mapper traits and fusion helpers between Pelorus Core / Stream payloads and chart snapshots.
//!
//! Each public type lives in its own file under `mapper/`; this `mod.rs` is just
//! the namespace assembly point.

mod core_sample_mapper;
mod fusion_clock;
mod stream_instant;
mod timed_ais_target;
mod timed_own_ship;
mod unconfigured_mapper;

pub use core_sample_mapper::CoreSampleMapper;
pub use fusion_clock::FusionClock;
pub use stream_instant::StreamInstant;
pub use timed_ais_target::TimedAisTarget;
pub use timed_own_ship::TimedOwnShip;
pub use unconfigured_mapper::UnconfiguredMapper;

use crate::OwnShip;

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
