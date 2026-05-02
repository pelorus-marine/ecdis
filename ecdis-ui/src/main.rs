//! Slint **Wayland-capable** shell (`winit` backend): load ENC, [`ChartNavContext`], **C2IL** outlines.
//!
//! Slint-generated UI code may use `unsafe` internally; this crate does not add additional `unsafe`
//! blocks in hand-written Rust.

slint::include_modules!();

use std::sync::{Arc, Mutex};

use ecdis_behaviours::{AlarmSink, NavAlertKind, display_is_overscaled_vs_chart_minimum};
use ecdis_portrayal::{
    ChartViewport, ChartViewportState, CpuOutlinePortrayal, UI_CHART_VIEWBOX_HEIGHT_PX,
    UI_CHART_VIEWBOX_WIDTH_PX, approx_own_ship_screen_px, demo_stub_segments_px,
};
use pelorus_core_adapter::{
    CoreSampleMapper, OwnShip, UnconfiguredMapper, merge_own_ship_fill_missing,
};
use pelorus_ecdis::ChartNavContext;
use s_101::{
    FcEditionSummary, FeatureCataloguePin, S101Dataset, TARGET_PRODUCT_SPECIFICATION_EDITION,
    parse_fc_edition_summary,
};
use slint::{ModelRc, VecModel};

#[derive(Clone)]
struct UiAlarmSink {
    ui: slint::Weak<AppWindow>,
}

impl AlarmSink for UiAlarmSink {
    fn emit(&mut self, kind: NavAlertKind, message: &str) {
        tracing::warn!(target: "ecdis.nav", kind = %kind, "{message}");
        let ui_w = self.ui.clone();
        let msg = format!("[{kind}] {message}\n");
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = ui_w.upgrade() {
                let cur = ui.get_alarm_log().to_string();
                ui.set_alarm_log((cur + &msg).into());
            }
        });
    }
}

fn segments_from_outline_or_stub(
    outline: Vec<(f32, f32, f32, f32)>,
    st: &ChartViewportState,
) -> Vec<StubSeg> {
    let iter: Box<dyn Iterator<Item = (f32, f32, f32, f32)>> = if outline.is_empty() {
        Box::new(
            demo_stub_segments_px(UI_CHART_VIEWBOX_WIDTH_PX, UI_CHART_VIEWBOX_HEIGHT_PX, st)
                .into_iter(),
        )
    } else {
        Box::new(outline.into_iter())
    };
    iter.map(|(x1, y1, x2, y2)| StubSeg { x1, y1, x2, y2 }).collect()
}

fn refresh_panel(
    ui: &AppWindow,
    ctx: &ChartNavContext,
    enc_path: &str,
    pin: &FeatureCataloguePin,
    fc: Option<&FcEditionSummary>,
    viewport: &Mutex<ChartViewport<CpuOutlinePortrayal>>,
) {
    let vp = viewport.lock().unwrap();
    let inv = ctx.chart.feature_inventory_with_pin(pin);

    ui.set_enc_path_label(format!("ENC: {enc_path}").into());

    let fc_note = fc
        .map(|f| {
            format!(
                "FC catalogue (XML): {} v{} ({})",
                f.product_id, f.version_number, f.version_date
            )
        })
        .unwrap_or_else(|| "FC catalogue (XML): (none — pass path as 2nd CLI arg)".to_string());

    ui.set_edition_label(
        format!(
            "Product spec edition pin: {} | FC edition pin: {:?}\n{fc_note}",
            pin.product_specification_edition, pin.feature_catalogue_edition,
        )
        .into(),
    );
    ui.set_inventory_label(
        format!(
            "FRID rows={} data_records={}",
            inv.records_with_frid, inv.total_data_records
        )
        .into(),
    );
    ui.set_own_ship_label(
        format!(
            "Own ship  lat={:?}  lon={:?}  SOG m/s={:?}  COG°={:?}  HDG°={:?}  depth m={:?}",
            ctx.own_ship.lat_deg,
            ctx.own_ship.lon_deg,
            ctx.own_ship.sog_mps,
            ctx.own_ship.cog_true_deg,
            ctx.own_ship.heading_true_deg,
            ctx.own_ship.depth_m,
        )
        .into(),
    );
    ui.set_vdr_status_label(ctx.vdr_status_line.as_str().into());
    ui.set_viewport_label(
        format!(
            "Viewport  centre λ={:.5}° φ={:.5}°  scale 1:{}  |  C2IL chains={}  CMF≈{}",
            vp.state.center_lon_deg,
            vp.state.center_lat_deg,
            vp.state.scale_denominator,
            vp.portrayal_ref().chain_count(),
            vp.portrayal_ref().crs().cmf_x,
        )
        .into(),
    );

    let vw = UI_CHART_VIEWBOX_WIDTH_PX;
    let vh = UI_CHART_VIEWBOX_HEIGHT_PX;
    let portrayal = vp.portrayal_ref();
    let outline_px = portrayal.segments_screen_px(&vp.state, vw, vh);
    let live_enc = !outline_px.is_empty();
    let outline_len = outline_px.len();
    let chains = portrayal.chain_count();

    let segs = segments_from_outline_or_stub(outline_px, &vp.state);

    let chart_visual_label = if live_enc {
        format!(
            "Chart ({}×{} px): ENC C2IL outlines — {} chain(s), {} line segment(s). Orange cross = own ship; cyan line = heading true when available.",
            vw as i32, vh as i32, chains, outline_len
        )
    } else {
        format!(
            "Chart ({}×{} px): demo stub (no C2IL in this cell). {} symbolic segments. Try IHO S-64 DisplayBase for real outlines. Orange cross = own ship; cyan = heading.",
            vw as i32,
            vh as i32,
            segs.len()
        )
    };
    ui.set_chart_visual_label(chart_visual_label.into());

    let mut own_ship_show = false;
    let mut own_ship_x = 0.0_f32;
    let mut own_ship_y = 0.0_f32;
    if let (Some(lat), Some(lon)) = (ctx.own_ship.lat_deg, ctx.own_ship.lon_deg) {
        let pos = if live_enc {
            portrayal
                .project_wgs84_to_screen_px(&vp.state, lat, lon, vw, vh)
                .unwrap_or_else(|| approx_own_ship_screen_px(&vp.state, lat, lon, vw, vh))
        } else {
            approx_own_ship_screen_px(&vp.state, lat, lon, vw, vh)
        };
        own_ship_x = pos.0;
        own_ship_y = pos.1;
        own_ship_show =
            pos.0 >= -48.0 && pos.0 <= vw + 48.0 && pos.1 >= -48.0 && pos.1 <= vh + 48.0;
    }
    ui.set_own_ship_marker_visible(own_ship_show);
    ui.set_own_ship_marker_x(own_ship_x);
    ui.set_own_ship_marker_y(own_ship_y);

    let mut heading_show = false;
    let mut hx2 = 0.0_f32;
    let mut hy2 = 0.0_f32;
    if own_ship_show {
        if let Some(hdg) = ctx.own_ship.heading_true_deg {
            let rad = hdg.to_radians();
            let len = 56.0_f64;
            hx2 = own_ship_x + (len * rad.sin()) as f32;
            hy2 = own_ship_y - (len * rad.cos()) as f32;
            heading_show = true;
        }
    }
    ui.set_own_ship_heading_visible(heading_show);
    ui.set_own_ship_heading_x2(hx2);
    ui.set_own_ship_heading_y2(hy2);

    ui.set_stub_segments(ModelRc::new(VecModel::from(segs)));
}

fn pixel_delta_to_deg(dx: f32, dy: f32, scale_denom: u32) -> (f64, f64) {
    let z = (f64::from(scale_denom) / 22_000.0).clamp(0.25, 10.0);
    let k = 0.00012 / z;
    (-f64::from(dx) * k, f64::from(dy) * k)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();

    let path = std::env::args()
        .nth(1)
        .ok_or("usage: ecdis-ui <path-to.enc.000> [feature-catalogue.xml]")?;

    let chart = S101Dataset::load(&path)?;

    let fc_summary = std::env::args()
        .nth(2)
        .and_then(|p| std::fs::read(p).ok())
        .and_then(|bytes| parse_fc_edition_summary(&bytes).ok());

    let mut pin = FeatureCataloguePin::default();
    if let Some(ref fc) = fc_summary {
        pin.feature_catalogue_edition = format!("{} {}", fc.version_number, fc.version_date);
    }

    let mapper = UnconfiguredMapper;
    let _ = CoreSampleMapper::map_own_ship(&mapper, &[]);

    let own_ship = merge_own_ship_fill_missing(
        OwnShip::with_position(51.0, 2.0),
        OwnShip {
            sog_mps: Some(3.0),
            heading_true_deg: Some(42.0),
            depth_m: Some(8.5),
            ..Default::default()
        },
    );

    let ctx = Arc::new(ChartNavContext::new(chart).with_own_ship(own_ship));

    let viewport = Arc::new(Mutex::new(ChartViewport::new(
        CpuOutlinePortrayal::default(),
    )));
    viewport.lock().unwrap().reset_chart(ctx.chart.as_ref())?;

    {
        let mut vp = viewport.lock().unwrap();
        vp.state.center_lat_deg = vp.portrayal_ref().anchor_lat_deg;
        vp.state.center_lon_deg = vp.portrayal_ref().anchor_lon_deg;
    }

    let ui = AppWindow::new()?;
    refresh_panel(
        &ui,
        ctx.as_ref(),
        &path,
        &pin,
        fc_summary.as_ref(),
        viewport.as_ref(),
    );

    let mut starter_log = format!(
        "S-101 decoder pin (product spec edition constant): {TARGET_PRODUCT_SPECIFICATION_EDITION}\n"
    );
    let mut alarms = UiAlarmSink { ui: ui.as_weak() };
    if display_is_overscaled_vs_chart_minimum(
        Some(12_000),
        viewport.lock().unwrap().state.scale_denominator,
    ) {
        alarms.emit(
            NavAlertKind::Overscale,
            "demo: mariner scale coarser than fictitious SCAMIN cap",
        );
    } else {
        starter_log.push_str("(no demo overscale alert)\n");
    }
    ui.set_alarm_log(starter_log.into());

    let vp_arc = viewport.clone();
    let ctx_arc = ctx.clone();
    let ui_weak = ui.as_weak();
    let path_owned = path.clone();
    let pin_arc = Arc::new(pin.clone());
    let fc_arc = Arc::new(fc_summary);

    let hook_zoom_pan = {
        let vp_arc = vp_arc.clone();
        let ctx_arc = ctx_arc.clone();
        let ui_weak = ui_weak.clone();
        let path_owned = path_owned.clone();
        let pin_arc = pin_arc.clone();
        let fc_arc = fc_arc.clone();
        move || {
            if let Some(ui) = ui_weak.upgrade() {
                refresh_panel(
                    &ui,
                    ctx_arc.as_ref(),
                    &path_owned,
                    pin_arc.as_ref(),
                    fc_arc.as_ref().as_ref(),
                    vp_arc.as_ref(),
                );
            }
        }
    };

    ui.on_zoom_in({
        let vp_arc = vp_arc.clone();
        let ctx_arc = ctx_arc.clone();
        let hook = hook_zoom_pan.clone();
        move || {
            let chart_ref = ctx_arc.chart.clone();
            let mut vp = vp_arc.lock().unwrap();
            let _ = vp.nudge_scale(chart_ref.as_ref(), 1.0 / 1.25);
            drop(vp);
            hook();
        }
    });

    ui.on_zoom_out({
        let vp_arc = vp_arc.clone();
        let ctx_arc = ctx_arc.clone();
        let hook = hook_zoom_pan.clone();
        move || {
            let chart_ref = ctx_arc.chart.clone();
            let mut vp = vp_arc.lock().unwrap();
            let _ = vp.nudge_scale(chart_ref.as_ref(), 1.25);
            drop(vp);
            hook();
        }
    });

    ui.on_pan_west({
        let vp_arc = vp_arc.clone();
        let hook = hook_zoom_pan.clone();
        move || {
            let mut vp = vp_arc.lock().unwrap();
            vp.pan_deg(-0.02, 0.0);
            drop(vp);
            hook();
        }
    });

    ui.on_pan_east({
        let vp_arc = vp_arc.clone();
        let hook = hook_zoom_pan.clone();
        move || {
            let mut vp = vp_arc.lock().unwrap();
            vp.pan_deg(0.02, 0.0);
            drop(vp);
            hook();
        }
    });

    ui.on_pan_north({
        let vp_arc = vp_arc.clone();
        let hook = hook_zoom_pan.clone();
        move || {
            let mut vp = vp_arc.lock().unwrap();
            vp.pan_deg(0.0, 0.02);
            drop(vp);
            hook();
        }
    });

    ui.on_pan_south({
        let vp_arc = vp_arc.clone();
        let hook = hook_zoom_pan.clone();
        move || {
            let mut vp = vp_arc.lock().unwrap();
            vp.pan_deg(0.0, -0.02);
            drop(vp);
            hook();
        }
    });

    ui.on_chart_pan_pixel({
        let vp_arc = vp_arc.clone();
        let hook = hook_zoom_pan.clone();
        move |dx, dy| {
            let mut vp = vp_arc.lock().unwrap();
            let sd = vp.state.scale_denominator;
            let (dlon, dlat) = pixel_delta_to_deg(dx, dy, sd);
            vp.pan_deg(dlon, dlat);
            drop(vp);
            hook();
        }
    });

    ui.on_chart_zoom_wheel({
        let vp_arc = vp_arc.clone();
        let ctx_arc = ctx_arc.clone();
        let hook = hook_zoom_pan.clone();
        move |delta_y| {
            let factor = if delta_y > 0.0 {
                1.2
            } else if delta_y < 0.0 {
                1.0 / 1.2
            } else {
                1.0
            };
            let chart_ref = ctx_arc.chart.clone();
            let mut vp = vp_arc.lock().unwrap();
            let _ = vp.nudge_scale(chart_ref.as_ref(), factor);
            drop(vp);
            hook();
        }
    });

    ui.run()?;
    Ok(())
}
