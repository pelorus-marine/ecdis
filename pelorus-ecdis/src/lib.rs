//! Combine **S-101 ENC** chart data with **Pelorus Core–oriented** navigation snapshots.
//!
//! This crate does **not** talk to CAN FD hardware; it defines **plain structs** and a small
//! [`ChartNavContext`] that an upstream service can populate from **Pelorus Core DCIDs** (position,
//! speed, depth, AIS, etc.) while the chart comes from [`s_101::S101Dataset`].
//!
//! See [ARCHITECTURE.md](https://github.com/pelorus-marine/ecdis/blob/main/pelorus-ecdis/ARCHITECTURE.md)
//! and the [Pelorus system architecture](https://github.com/pelorus-marine/specifications/blob/main/ARCHITECTURE.md).

#![forbid(unsafe_code)]

mod ais;
mod bridge;
mod own_ship;

pub use ais::AisVesselReport;
pub use bridge::ChartNavContext;
pub use own_ship::OwnShip;

#[doc(inline)]
pub use pelorus_core::OwnShipSnapshot;
