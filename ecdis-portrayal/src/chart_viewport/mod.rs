//! Stub **chart viewport** — pan/zoom state tied to [`crate::PortrayalPipeline`] scale hooks.
//!
//! Projection is **non-navigational** (dashboard-grade); ENC geometry portrayal remains future work.

mod chart_viewport;
mod chart_viewport_state;

pub use chart_viewport::ChartViewport;
pub use chart_viewport_state::ChartViewportState;

/// Horizontal extent of the UI chart `Path` viewbox — keep [`UI_CHART_VIEWBOX_HEIGHT_PX`] aligned with Slint (`ecdis-ui/ui/app.slint`).
pub const UI_CHART_VIEWBOX_WIDTH_PX: f32 = 880.0;
/// Vertical extent of the UI chart `Path` viewbox — keep in sync with Slint.
pub const UI_CHART_VIEWBOX_HEIGHT_PX: f32 = 420.0;

/// Flat-earth screen projection when **no** ENC chains drive scale (stub / empty C2IL).
#[must_use]
pub fn approx_own_ship_screen_px(
    vp: &ChartViewportState,
    lat_deg: f64,
    lon_deg: f64,
    width_px: f32,
    height_px: f32,
) -> (f32, f32) {
    let w = f64::from(width_px.max(32.0));
    let h = f64::from(height_px.max(32.0));
    let cos = vp.center_lat_deg.to_radians().cos().abs().max(0.12);
    let scale = (22_000f64 / f64::from(vp.scale_denominator.max(500))).sqrt().clamp(0.35, 8.0)
        * (w.min(h) / 6.5);
    let x = ((lon_deg - vp.center_lon_deg) * scale * cos + w * 0.5) as f32;
    let y = (h * 0.5 - (lat_deg - vp.center_lat_deg) * scale) as f32;
    (x, y)
}

/// Demo segments in **chart-area pixels** `(x1, y1, x2, y2)` for UI stubs (e.g. Slint `Line`).
#[must_use]
pub fn demo_stub_segments_px(
    width_px: f32,
    height_px: f32,
    state: &ChartViewportState,
) -> Vec<(f32, f32, f32, f32)> {
    let w = width_px.max(32.0);
    let h = height_px.max(32.0);
    let pad = 28.0_f32;
    let mut out = Vec::with_capacity(6);

    // Border
    out.push((pad, pad, w - pad, pad));
    out.push((w - pad, pad, w - pad, h - pad));
    out.push((w - pad, h - pad, pad, h - pad));
    out.push((pad, h - pad, pad, pad));

    let cx = w * 0.5;
    let cy = h * 0.5;
    let frac = (state.scale_denominator.min(2_000_000) as f32 / 22_000.0)
        .sqrt()
        .clamp(0.35, 8.0);
    let arm = (pad * 2.5 * frac).min(w.min(h) * 0.35);

    out.push((cx - arm, cy, cx + arm, cy));
    out.push((cx, cy - arm, cx, cy + arm));

    let jitter_lon = ((state.center_lon_deg * 1_000.0).sin() as f32 * 6.0).clamp(-18.0, 18.0);
    let jitter_lat = ((state.center_lat_deg * 900.0).sin() as f32 * 6.0).clamp(-18.0, 18.0);
    out.push((
        cx + jitter_lon,
        cy + jitter_lat,
        cx + jitter_lon + 48.0,
        cy + jitter_lat - 32.0,
    ));

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::portrayal::NoPortrayal;
    use s_101::S101Dataset;

    #[test]
    fn viewport_track_scale_and_reset_chart_when_fixture_present() {
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../testdata/s101_sample.000");
        if !path.exists() {
            return;
        }
        let chart = S101Dataset::load(path).unwrap();
        let mut vp = ChartViewport::new(NoPortrayal);
        vp.reset_chart(&chart).unwrap();
        vp.nudge_scale(&chart, 2.0).unwrap();
        assert!(vp.state.scale_denominator > 22_000);
        vp.pan_deg(0.01, -0.02);
        assert_ne!(vp.state.center_lon_deg, 2.0);
    }

    #[test]
    fn demo_segments_nonempty() {
        let segs = demo_stub_segments_px(400.0, 300.0, &ChartViewportState::default());
        assert!(!segs.is_empty());
    }
}
