//! **S-101 ENC** chart data plus **Pelorus Core–oriented** navigation snapshots and Core/Stream mapper hooks.
//!
//! Combines [`ChartNavContext`] / [`OwnShip`] / [`AisVesselReport`] with [`CoreSampleMapper`] and fusion
//! helpers. No CAN FD, NMEA, or sockets in this crate.
//!
//! See [ARCHITECTURE.md](ARCHITECTURE.md) and the
//! [Pelorus system architecture](https://github.com/pelorus-marine/specifications/blob/main/ARCHITECTURE.md).

#![forbid(unsafe_code)]

mod ais;
mod bridge;
mod mapper;
mod own_ship;

pub use ais::AisVesselReport;
pub use bridge::ChartNavContext;
pub use mapper::{
    CoreSampleMapper, FusionClock, StreamInstant, TimedAisTarget, TimedOwnShip, UnconfiguredMapper,
    merge_own_ship_fill_missing,
};
pub use own_ship::OwnShip;

#[doc(inline)]
pub use pelorus_core::OwnShipSnapshot;
