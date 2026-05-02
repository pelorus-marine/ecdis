use std::sync::Arc;

use pelorus_core::OwnShipSnapshot;
use s_101::S101Dataset;

use crate::{AisVesselReport, OwnShip};

/// Holds an ENC dataset plus the **dynamic** inputs needed for ECDIS-class logic.
///
/// Rendering, alerts, and UKC computations stay **outside** this struct; it is a convenient
/// bundle for services bridging **Pelorus Core** telemetry and **S-101** chart bytes.
pub struct ChartNavContext {
    pub chart: Arc<S101Dataset>,
    pub own_ship: OwnShip,
    pub ais_targets: Vec<AisVesselReport>,
    /// Voyage recorder status line for ECDIS HUD (integrate [`pelorus_vdr`](https://github.com/pelorus-marine/platform/tree/main/pelorus-vdr) when deployed).
    pub vdr_status_line: String,
}

impl ChartNavContext {
    pub fn new(chart: S101Dataset) -> Self {
        Self {
            chart: Arc::new(chart),
            own_ship: OwnShip::default(),
            ais_targets: Vec::new(),
            vdr_status_line: String::from("VDR: offline (run pelorus-vdr service when integrated)"),
        }
    }

    pub fn with_own_ship(mut self, own_ship: OwnShip) -> Self {
        self.own_ship = own_ship;
        self
    }

    /// Convenience: convert [`OwnShipSnapshot`](pelorus_core::OwnShipSnapshot) into chart-ready [`OwnShip`].
    pub fn with_own_ship_snapshot(mut self, snap: OwnShipSnapshot) -> Self {
        self.own_ship = snap.into();
        self
    }

    pub fn with_ais_targets(mut self, targets: Vec<AisVesselReport>) -> Self {
        self.ais_targets = targets;
        self
    }

    pub fn with_vdr_status_line(mut self, line: impl Into<String>) -> Self {
        self.vdr_status_line = line.into();
        self
    }

    pub fn chart_record_count(&self) -> usize {
        self.chart.record_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::OwnShipSnapshot;
    use s_101::S101Dataset;

    #[test]
    fn context_tracks_records_and_ship() {
        // Chart may not load without fixture; build from placeholder path only where we skip
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../testdata/s101_sample.000");
        if !path.exists() {
            return;
        }
        let chart = S101Dataset::load(path).unwrap();
        let ctx = ChartNavContext::new(chart).with_own_ship(OwnShip::with_position(51.0, 2.0));
        assert!(ctx.chart_record_count() > 0);
        assert_eq!(ctx.own_ship.lat_deg, Some(51.0));
    }

    #[test]
    fn context_accepts_own_ship_snapshot() {
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../testdata/s101_sample.000");
        if !path.exists() {
            return;
        }
        let chart = S101Dataset::load(path).unwrap();
        let snap = OwnShipSnapshot::with_position(12.5, -44.25);
        let ctx = ChartNavContext::new(chart).with_own_ship_snapshot(snap);
        assert_eq!(ctx.own_ship.lat_deg, Some(12.5));
        assert_eq!(ctx.own_ship.lon_deg, Some(-44.25));
    }
}
