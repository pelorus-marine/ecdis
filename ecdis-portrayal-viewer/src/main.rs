//! Developer-only Slint gallery for [`ecdis_portrayal`] frame builders.

mod viewer_ui;

use std::path::PathBuf;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use ecdis_portrayal::{
    ChartViewport, CpuOutlinePortrayal, DisplayMode, PortrayalFrame, PortrayalInputs, ViewerScene,
    build_frame, open_portrayal_catalogue_zip, open_s101_feature_catalogue_from_s64_zip,
    open_s101_portrayal_from_s64_zip,
};
use s_101::{FeatureCatalogue, PortrayalCatalogueBundle, S101Dataset};
use slint::ComponentHandle;
use viewer_ui::{ViewerWindow, apply_frame};

static REFRESH_GENERATION: AtomicU64 = AtomicU64::new(0);

struct ViewerState {
    enc_path: String,
    chart: S101Dataset,
    feature_catalogue: Option<FeatureCatalogue>,
    catalogue: Option<PortrayalCatalogueBundle>,
    viewport: ChartViewport<CpuOutlinePortrayal>,
    scene: ViewerScene,
    display_mode: DisplayMode,
    symbol_index: usize,
}

#[derive(Clone)]
struct ViewerCli {
    display_mode: DisplayMode,
    s64_zip: Option<PathBuf>,
    portrayal_catalogue: Option<PathBuf>,
}

impl Default for ViewerCli {
    fn default() -> Self {
        Self {
            display_mode: DisplayMode::Day,
            s64_zip: None,
            portrayal_catalogue: None,
        }
    }
}

struct UiRefresh {
    generation: u64,
    frame: PortrayalFrame,
    scene_index: i32,
    catalogue_label: String,
    viewport_label: String,
    symbol_ids: Vec<slint::SharedString>,
    symbol_index: i32,
}

fn scene_from_index(i: i32) -> ViewerScene {
    match i {
        1 => ViewerScene::FeatureGraph,
        2 => ViewerScene::SymbolGallery,
        3 => ViewerScene::ThemeSwatches,
        _ => ViewerScene::Chart,
    }
}

fn scene_to_index(scene: ViewerScene) -> i32 {
    match scene {
        ViewerScene::Chart => 0,
        ViewerScene::FeatureGraph => 1,
        ViewerScene::SymbolGallery => 2,
        ViewerScene::ThemeSwatches => 3,
    }
}

fn pixel_delta_to_deg(dx: f32, dy: f32, scale_denom: u32) -> (f64, f64) {
    let z = (f64::from(scale_denom) / 22_000.0).clamp(0.25, 10.0);
    let k = 0.00012 / z;
    (-f64::from(dx) * k, f64::from(dy) * k)
}

fn s64_zip_path(cli: &ViewerCli) -> Option<PathBuf> {
    cli.s64_zip.clone().or_else(|| {
        let default = PathBuf::from("target/iho-cache/S-64_1.2.0.zip");
        default.is_file().then_some(default)
    })
}

fn load_feature_catalogue(
    fc_path: Option<&PathBuf>,
    cli: &ViewerCli,
) -> Result<Option<FeatureCatalogue>, Box<dyn std::error::Error>> {
    if let Some(p) = fc_path {
        let bytes = std::fs::read(p)?;
        return Ok(Some(FeatureCatalogue::parse_xml(&bytes)?));
    }
    if let Some(p) = s64_zip_path(cli) {
        match open_s101_feature_catalogue_from_s64_zip(&p) {
            Ok(fc) => return Ok(fc),
            Err(e) => tracing::warn!("S-64 feature catalogue load failed: {e}"),
        }
    }
    Ok(None)
}

fn parse_args() -> Result<(PathBuf, Option<PathBuf>, ViewerCli), Box<dyn std::error::Error>> {
    let mut enc: Option<PathBuf> = None;
    let mut fc: Option<PathBuf> = None;
    let mut cli = ViewerCli::default();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        if let Some(v) = a.strip_prefix("--display-mode=") {
            cli.display_mode = DisplayMode::parse(v).ok_or("invalid --display-mode")?;
        } else if a == "--s64-zip" {
            cli.s64_zip = Some(args.next().ok_or("--s64-zip requires path")?.into());
        } else if let Some(v) = a.strip_prefix("--portrayal-catalogue=") {
            cli.portrayal_catalogue = Some(v.into());
        } else if a.starts_with('-') {
            return Err(format!("unknown flag: {a}").into());
        } else if enc.is_none() {
            enc = Some(a.into());
        } else if fc.is_none() {
            fc = Some(a.into());
        } else {
            return Err("too many positional arguments".into());
        }
    }
    let enc = enc.ok_or(
        "usage: ecdis-portrayal-viewer <cell.000> [feature_catalogue.xml] [--s64-zip path] [--portrayal-catalogue=path] [--display-mode=day|dusk|night]",
    )?;
    Ok((enc, fc, cli))
}

fn load_catalogue(cli: &ViewerCli) -> Result<Option<PortrayalCatalogueBundle>, Box<dyn std::error::Error>> {
    if let Some(ref p) = cli.portrayal_catalogue {
        return Ok(Some(open_portrayal_catalogue_zip(p)?));
    }
    if let Some(ref p) = cli.s64_zip {
        return Ok(Some(open_s101_portrayal_from_s64_zip(p)?));
    }
    let default = PathBuf::from("target/iho-cache/S-64_1.2.0.zip");
    if default.is_file() {
        return Ok(Some(open_s101_portrayal_from_s64_zip(&default)?));
    }
    Ok(None)
}

fn catalogue_label(bundle: Option<&PortrayalCatalogueBundle>, mode: DisplayMode) -> String {
    let Some(b) = bundle else {
        return format!("Catalogue: (none) — fallback theme for {}", mode.palette_name());
    };
    let palettes = b.palette_names();
    let css = b
        .catalogue
        .palette(mode.palette_name())
        .and_then(|p| p.css.as_deref())
        .unwrap_or("(no css)");
    format!(
        "Catalogue: {} v{} | palettes={palettes:?} | active={} css={css}",
        b.catalogue.manifest.product_id,
        b.catalogue.manifest.version,
        mode.palette_name()
    )
}

fn build_ui_refresh(state: &ViewerState) -> UiRefresh {
    let selected_symbol = state.catalogue.as_ref().and_then(|b| {
        b.catalogue
            .manifest
            .symbols
            .get(state.symbol_index % b.catalogue.manifest.symbols.len().max(1))
            .map(|s| s.id.as_str())
    });

    let inputs = PortrayalInputs {
        chart: &state.chart,
        feature_catalogue: state.feature_catalogue.as_ref(),
        catalogue: state.catalogue.as_ref(),
        viewport: &state.viewport.state,
        display_mode: state.display_mode,
        outline: state.viewport.portrayal_ref(),
        selected_symbol_id: selected_symbol,
    };

    let frame = build_frame(&inputs, state.scene);
    let symbol_ids = state
        .catalogue
        .as_ref()
        .map(|b| {
            b.catalogue
                .manifest
                .symbols
                .iter()
                .map(|s| s.id.as_str().into())
                .collect()
        })
        .unwrap_or_default();

    let vp = &state.viewport.state;
    let viewport_label = format!(
        "Viewport  centre λ={:.5}° φ={:.5}°  scale 1:{}  |  C2IL chains={}",
        vp.center_lon_deg,
        vp.center_lat_deg,
        vp.scale_denominator,
        state.viewport.portrayal_ref().chain_count(),
    );

    UiRefresh {
        generation: 0,
        frame,
        scene_index: scene_to_index(state.scene),
        catalogue_label: catalogue_label(state.catalogue.as_ref(), state.display_mode),
        viewport_label,
        symbol_ids,
        symbol_index: state.symbol_index as i32,
    }
}

fn apply_ui_refresh(ui: &ViewerWindow, update: UiRefresh) {
    apply_frame(ui, &update.frame);
    ui.set_scene_index(update.scene_index);
    ui.set_catalogue_label(update.catalogue_label.into());
    ui.set_viewport_label(update.viewport_label.into());
    ui.set_symbol_ids(slint::ModelRc::new(slint::VecModel::from(update.symbol_ids)));
    ui.set_symbol_index(update.symbol_index);
}

/// Run frame build off the UI thread so Slint stays responsive.
fn schedule_refresh(state: Arc<Mutex<ViewerState>>, ui_weak: slint::Weak<ViewerWindow>) {
    let generation = REFRESH_GENERATION.fetch_add(1, Ordering::SeqCst) + 1;
    std::thread::spawn(move || {
        let mut update = {
            let st = state.lock().expect("viewer state");
            build_ui_refresh(&st)
        };
        update.generation = generation;
        let _ = slint::invoke_from_event_loop(move || {
            if REFRESH_GENERATION.load(Ordering::SeqCst) != generation {
                return;
            }
            if let Some(ui) = ui_weak.upgrade() {
                apply_ui_refresh(&ui, update);
            }
        });
    });
}

fn schedule_catalogue_load(
    state: Arc<Mutex<ViewerState>>,
    ui_weak: slint::Weak<ViewerWindow>,
    cli: ViewerCli,
) {
    std::thread::spawn(move || {
        let catalogue = match load_catalogue(&cli) {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!("portrayal catalogue load failed: {e}");
                None
            }
        };
        {
            let mut st = state.lock().expect("viewer state");
            st.catalogue = catalogue;
        }
        schedule_refresh(state, ui_weak);
    });
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();

    let (enc_path, fc_path, cli) = parse_args()?;
    let chart = S101Dataset::load(&enc_path)?;
    let feature_catalogue = load_feature_catalogue(fc_path.as_ref(), &cli)?;

    let mut viewport = ChartViewport::new(CpuOutlinePortrayal::default());
    viewport.reset_chart(&chart)?;
    viewport.state.center_lat_deg = viewport.portrayal_ref().anchor_lat_deg;
    viewport.state.center_lon_deg = viewport.portrayal_ref().anchor_lon_deg;

    let state = Arc::new(Mutex::new(ViewerState {
        enc_path: enc_path.display().to_string(),
        chart,
        feature_catalogue,
        catalogue: None,
        viewport,
        scene: ViewerScene::Chart,
        display_mode: cli.display_mode,
        symbol_index: 0,
    }));

    let ui = ViewerWindow::new()?;
    ui.set_enc_label(format!("ENC: {}", state.lock().unwrap().enc_path).into());
    ui.set_caption_label("Loading portrayal catalogue…".into());
    ui.set_catalogue_label("Catalogue: loading…".into());

    let ui_weak = ui.as_weak();

    // Fast first paint (C2IL / stub) without waiting for the portrayal catalogue zip.
    schedule_refresh(state.clone(), ui_weak.clone());
    schedule_catalogue_load(state.clone(), ui_weak.clone(), cli);

    let hook: Rc<dyn Fn()> = Rc::new({
        let state_w = state.clone();
        let ui_w = ui_weak.clone();
        move || schedule_refresh(state_w.clone(), ui_w.clone())
    });

    ui.on_scene_selected({
        let state_w = state.clone();
        let hook = Rc::clone(&hook);
        move |idx| {
            state_w.lock().unwrap().scene = scene_from_index(idx);
            hook();
        }
    });

    ui.on_display_day({
        let state_w = state.clone();
        let hook = Rc::clone(&hook);
        move || {
            state_w.lock().unwrap().display_mode = DisplayMode::Day;
            hook();
        }
    });
    ui.on_display_dusk({
        let state_w = state.clone();
        let hook = Rc::clone(&hook);
        move || {
            state_w.lock().unwrap().display_mode = DisplayMode::Dusk;
            hook();
        }
    });
    ui.on_display_night({
        let state_w = state.clone();
        let hook = Rc::clone(&hook);
        move || {
            state_w.lock().unwrap().display_mode = DisplayMode::Night;
            hook();
        }
    });

    ui.on_symbol_prev({
        let state_w = state.clone();
        let hook = Rc::clone(&hook);
        move || {
            let mut st = state_w.lock().unwrap();
            if st.catalogue.is_some() {
                let n = st
                    .catalogue
                    .as_ref()
                    .map(|b| b.catalogue.manifest.symbols.len())
                    .unwrap_or(1);
                st.symbol_index = (st.symbol_index + n - 1) % n;
            }
            hook();
        }
    });

    ui.on_symbol_next({
        let state_w = state.clone();
        let hook = Rc::clone(&hook);
        move || {
            let mut st = state_w.lock().unwrap();
            if st.catalogue.is_some() {
                let n = st
                    .catalogue
                    .as_ref()
                    .map(|b| b.catalogue.manifest.symbols.len())
                    .unwrap_or(1);
                st.symbol_index = (st.symbol_index + 1) % n;
            }
            hook();
        }
    });

    ui.on_chart_zoom_in({
        let state_w = state.clone();
        let hook = Rc::clone(&hook);
        move || {
            let mut st = state_w.lock().unwrap();
            let _ = st.viewport.nudge_scale_factor(1.0 / 1.25);
            hook();
        }
    });
    ui.on_chart_zoom_out({
        let state_w = state.clone();
        let hook = Rc::clone(&hook);
        move || {
            let mut st = state_w.lock().unwrap();
            let _ = st.viewport.nudge_scale_factor(1.25);
            hook();
        }
    });
    ui.on_chart_zoom_wheel({
        let state_w = state.clone();
        let hook = Rc::clone(&hook);
        move |delta_y| {
            if delta_y == 0.0 {
                return;
            }
            let factor = if delta_y < 0.0 { 1.0 / 1.12 } else { 1.12 };
            let mut st = state_w.lock().unwrap();
            let _ = st.viewport.nudge_scale_factor(factor);
            hook();
        }
    });
    ui.on_chart_pan_pixel({
        let state_w = state.clone();
        let hook = Rc::clone(&hook);
        move |dx, dy| {
            let mut st = state_w.lock().unwrap();
            let scale = st.viewport.state.scale_denominator;
            let (dlon, dlat) = pixel_delta_to_deg(dx, dy, scale);
            st.viewport.pan_deg(dlon, dlat);
            hook();
        }
    });
    let pan_step = 0.004;
    ui.on_pan_north({
        let state_w = state.clone();
        let hook = Rc::clone(&hook);
        move || {
            state_w.lock().unwrap().viewport.pan_deg(0.0, pan_step);
            hook();
        }
    });
    ui.on_pan_south({
        let state_w = state.clone();
        let hook = Rc::clone(&hook);
        move || {
            state_w.lock().unwrap().viewport.pan_deg(0.0, -pan_step);
            hook();
        }
    });
    ui.on_pan_west({
        let state_w = state.clone();
        let hook = Rc::clone(&hook);
        move || {
            state_w.lock().unwrap().viewport.pan_deg(-pan_step, 0.0);
            hook();
        }
    });
    ui.on_pan_east({
        let state_w = state.clone();
        let hook = Rc::clone(&hook);
        move || {
            state_w.lock().unwrap().viewport.pan_deg(pan_step, 0.0);
            hook();
        }
    });

    ui.run()?;
    Ok(())
}
