//! Minimal **composition root**: load **S-101**, build [`ChartNavContext`], exercise portrayal + behaviour stubs.
//!
//! Not a certified ECDIS — wiring demo for Pelorus + Stream integration paths.

#![forbid(unsafe_code)]

use ecdis_behaviours::{
    display_is_overscaled_vs_chart_minimum, AlarmSink, NavAlertKind, StderrAlarmSink,
};
use ecdis_portrayal::{NoPortrayal, PortrayalPipeline};
use pelorus_core_adapter::{
    merge_own_ship_fill_missing, CoreSampleMapper, OwnShip, UnconfiguredMapper,
};
use pelorus_ecdis::ChartNavContext;
use s_101::{FeatureCataloguePin, S101Dataset, TARGET_PRODUCT_SPECIFICATION_EDITION};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .ok_or("usage: cargo run -p ecdis-runtime -- <path-to.enc.000>")?;

    let chart = S101Dataset::load(&path)?;
    let pin = FeatureCataloguePin::default();
    let inv = chart.feature_inventory_with_pin(&pin);

    println!("S-101 decoder pin (product spec edition): {TARGET_PRODUCT_SPECIFICATION_EDITION}");
    println!("Feature catalogue pin: {:?}", pin.feature_catalogue_edition);
    println!(
        "Feature inventory: records_with_frid={} total_data_records={}",
        inv.records_with_frid, inv.total_data_records
    );

    let mapper = UnconfiguredMapper;
    let _ = CoreSampleMapper::map_own_ship(&mapper, &[]);

    let ship = merge_own_ship_fill_missing(
        OwnShip::with_position(51.0, 2.0),
        OwnShip {
            sog_mps: Some(3.0),
            ..Default::default()
        },
    );

    let ctx = ChartNavContext::new(chart).with_own_ship(ship);

    let mut portray = NoPortrayal;
    portray.reset_for_chart(&ctx.chart)?;
    portray.set_display_scale(22_000)?;

    let mut alarms = StderrAlarmSink;
    if display_is_overscaled_vs_chart_minimum(Some(12_000), 22_000) {
        alarms.emit(
            NavAlertKind::Overscale,
            "demo: mariner scale coarser than fictitious SCAMIN cap",
        );
    }

    println!(
        "ChartNavContext: chart records={}, own_ship lat={:?}",
        ctx.chart_record_count(),
        ctx.own_ship.lat_deg
    );

    Ok(())
}
