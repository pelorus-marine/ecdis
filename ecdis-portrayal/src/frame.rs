//! UI-agnostic portrayal output (segments, swatches, symbol bitmaps).

use s_101::{FeatureCatalogue, PortrayalCatalogueBundle, S101Dataset};

use crate::chart_theme::{ChartTheme, Rgb};
use crate::chart_viewport::{ChartViewportState, UI_CHART_VIEWBOX_HEIGHT_PX, UI_CHART_VIEWBOX_WIDTH_PX};
use crate::cpu_outline::CpuOutlinePortrayal;
use crate::display_mode::DisplayMode;
use crate::feature_graph_frame;

#[cfg(feature = "symbols")]
use crate::symbol_render;

/// Gallery scenes exposed by the portrayal viewer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewerScene {
    /// ENC chart view: C2IL coastline outlines plus optional FC-resolved geometry.
    #[default]
    Chart,
    FeatureGraph,
    SymbolGallery,
    ThemeSwatches,
}

/// Inputs shared by frame builders.
pub struct PortrayalInputs<'a> {
    pub chart: &'a S101Dataset,
    pub feature_catalogue: Option<&'a FeatureCatalogue>,
    pub catalogue: Option<&'a PortrayalCatalogueBundle>,
    pub viewport: &'a ChartViewportState,
    pub display_mode: DisplayMode,
    pub outline: &'a CpuOutlinePortrayal,
    /// Selected symbol id for [`ViewerScene::SymbolGallery`].
    pub selected_symbol_id: Option<&'a str>,
}

/// One screen line segment.
#[derive(Debug, Clone, Copy)]
pub struct LineSegment {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub stroke: Rgb,
    pub width_px: f32,
}

/// Closed polygon for area portrayal.
#[derive(Debug, Clone)]
pub struct FilledPolygon {
    pub vertices: Vec<(f32, f32)>,
    pub fill: Rgb,
    pub stroke: Rgb,
    pub fill_alpha: f32,
}

/// Point marker.
#[derive(Debug, Clone, Copy)]
pub struct PointMarker {
    pub x: f32,
    pub y: f32,
    pub radius_px: f32,
    pub fill: Rgb,
    pub stroke: Rgb,
}

/// Colour token swatch for the theme scene.
#[derive(Debug, Clone)]
pub struct ColorSwatch {
    pub token: String,
    pub rgb: Rgb,
    pub x: f32,
    pub y: f32,
    pub size_px: f32,
}

/// Rasterized catalogue symbol.
#[derive(Debug, Clone)]
pub struct SymbolSprite {
    pub symbol_id: String,
    pub x: f32,
    pub y: f32,
    pub width_px: u32,
    pub height_px: u32,
    pub rgba: Vec<u8>,
}

/// Drawable layers for one frame.
#[derive(Debug, Clone, Default)]
pub struct PortrayalLayers {
    pub segments: Vec<LineSegment>,
    pub polygons: Vec<FilledPolygon>,
    pub points: Vec<PointMarker>,
    pub swatches: Vec<ColorSwatch>,
    pub symbols: Vec<SymbolSprite>,
    pub caption: String,
}

/// Full chart frame in logical pixels.
#[derive(Debug, Clone)]
pub struct PortrayalFrame {
    pub width_px: f32,
    pub height_px: f32,
    pub theme: ChartTheme,
    pub layers: PortrayalLayers,
}

impl PortrayalFrame {
    fn new(theme: ChartTheme, layers: PortrayalLayers) -> Self {
        Self {
            width_px: UI_CHART_VIEWBOX_WIDTH_PX,
            height_px: UI_CHART_VIEWBOX_HEIGHT_PX,
            theme,
            layers,
        }
    }
}

fn active_palette<'a>(
    inputs: &'a PortrayalInputs<'a>,
) -> Option<&'a s_101::ColorPalette> {
    inputs
        .catalogue
        .and_then(|b| b.catalogue.palette(inputs.display_mode.palette_name()))
}

/// ENC chart frame: **C2IL** coastline polylines only (same basis as [`ecdis-ui`](../ecdis-ui/)).
///
/// Full S-101 portrayal (symbols, area fills, rules) is not executed here. Use
/// [`ViewerScene::FeatureGraph`] for FC-resolved geometry in the same projection when C2IL is present.
pub fn build_chart_frame(inputs: &PortrayalInputs<'_>) -> PortrayalFrame {
    let mut frame = build_c2il_outline_frame(inputs);
    let live = frame.layers.caption.starts_with("C2IL outline");
    frame.layers.caption = if live {
        format!(
            "Chart — {}\nCoastline from ENC C2IL (not full portrayal catalogue). Feature geometry: Feature graph scene.",
            frame.layers.caption
        )
    } else {
        format!(
            "Chart — {}\nTry IHO S-64 DisplayBase for real ENC outlines.",
            frame.layers.caption
        )
    };
    frame
}

/// C2IL outline segments (or demo stub when empty).
pub fn build_c2il_outline_frame(inputs: &PortrayalInputs<'_>) -> PortrayalFrame {
    let theme = ChartTheme::resolve(inputs.display_mode, active_palette(inputs));
    let w = UI_CHART_VIEWBOX_WIDTH_PX;
    let h = UI_CHART_VIEWBOX_HEIGHT_PX;
    let mut segments = inputs
        .outline
        .segments_screen_px(inputs.viewport, w, h)
        .into_iter()
        .map(|(x1, y1, x2, y2)| LineSegment {
            x1,
            y1,
            x2,
            y2,
            stroke: theme.chart_stroke,
            width_px: 1.5,
        })
        .collect::<Vec<_>>();

    let live = !segments.is_empty();
    if !live {
        segments = crate::chart_viewport::demo_stub_segments_px(w, h, inputs.viewport)
            .into_iter()
            .map(|(x1, y1, x2, y2)| LineSegment {
                x1,
                y1,
                x2,
                y2,
                stroke: theme.chart_stroke,
                width_px: 2.0,
            })
            .collect();
    }

    let caption = if live {
        format!(
            "C2IL outline — {} chain(s), {} segment(s), scale 1:{}",
            inputs.outline.chain_count(),
            segments.len(),
            inputs.viewport.scale_denominator
        )
    } else {
        format!(
            "Demo stub — {} segment(s), scale 1:{} (no C2IL in cell)",
            segments.len(),
            inputs.viewport.scale_denominator
        )
    };

    PortrayalFrame::new(
        theme,
        PortrayalLayers {
            segments,
            caption,
            ..Default::default()
        },
    )
}

/// FC-resolved feature geometry.
pub fn build_feature_graph_frame(inputs: &PortrayalInputs<'_>) -> PortrayalFrame {
    let theme = ChartTheme::resolve(inputs.display_mode, active_palette(inputs));
    let Some(fc) = inputs.feature_catalogue else {
        return PortrayalFrame::new(
            theme,
            PortrayalLayers {
                caption: "Feature graph — pass S-101 feature catalogue XML path".to_string(),
                ..Default::default()
            },
        );
    };

    let graph = match inputs.chart.build_feature_graph(fc) {
        Ok(g) => g,
        Err(e) => {
            return PortrayalFrame::new(
                theme,
                PortrayalLayers {
                    caption: format!("Feature graph build error: {e}"),
                    ..Default::default()
                },
            );
        }
    };

    let layers = feature_graph_frame::layers_from_graph(
        &graph,
        inputs.viewport,
        UI_CHART_VIEWBOX_WIDTH_PX,
        UI_CHART_VIEWBOX_HEIGHT_PX,
        &theme,
        Some(inputs.outline),
    );
    PortrayalFrame::new(theme, layers)
}

/// Token swatches for the active display mode.
pub fn build_theme_swatches_frame(inputs: &PortrayalInputs<'_>) -> PortrayalFrame {
    let palette = active_palette(inputs);
    let theme = ChartTheme::resolve(inputs.display_mode, palette);
    let mut swatches = Vec::new();
    let cols = 8_usize;
    let size = 28.0_f32;
    let gap = 6.0_f32;
    let origin_x = 12.0_f32;
    let origin_y = 36.0_f32;

    if let Some(p) = palette {
        for (i, item) in p.items.iter().enumerate() {
            let col = i % cols;
            let row = i / cols;
            swatches.push(ColorSwatch {
                token: item.token.clone(),
                rgb: Rgb::new(item.srgb.0, item.srgb.1, item.srgb.2),
                x: origin_x + (size + gap) * col as f32,
                y: origin_y + (size + gap) * row as f32,
                size_px: size,
            });
        }
    } else {
        for (i, (token, rgb)) in fallback_swatch_tokens(inputs.display_mode).iter().enumerate() {
            let col = i % cols;
            let row = i / cols;
            swatches.push(ColorSwatch {
                token: (*token).to_string(),
                rgb: *rgb,
                x: origin_x + (size + gap) * col as f32,
                y: origin_y + (size + gap) * row as f32,
                size_px: size,
            });
        }
    }

    let css_note = theme
        .palette_css
        .as_deref()
        .map(|c| format!("stylesheet: {c}"))
        .unwrap_or_else(|| "stylesheet: (fallback — no catalogue)".to_string());

    let caption = format!(
        "Theme swatches — {} mode, {} token(s), {css_note}, catalogue={}",
        inputs.display_mode.palette_name(),
        swatches.len(),
        theme.from_catalogue
    );

    PortrayalFrame::new(
        theme,
        PortrayalLayers {
            swatches,
            caption,
            ..Default::default()
        },
    )
}

fn fallback_swatch_tokens(mode: DisplayMode) -> [(&'static str, Rgb); 6] {
    let t = ChartTheme::resolve(mode, None);
    [
        ("DEPDW", t.background),
        ("CSTLN", t.chart_stroke),
        ("CHYLW", t.own_ship),
        ("CHBLK", t.hud_primary),
        ("CHGRD", t.hud_muted),
        ("CHMGD", t.heading),
    ]
}

/// Rasterized symbol(s) from the portrayal catalogue.
#[cfg(feature = "symbols")]
pub fn build_symbol_gallery_frame(inputs: &PortrayalInputs<'_>) -> PortrayalFrame {
    let theme = ChartTheme::resolve(inputs.display_mode, active_palette(inputs));
    let Some(bundle) = inputs.catalogue else {
        return PortrayalFrame::new(
            theme,
            PortrayalLayers {
                caption: "Symbol gallery — load portrayal catalogue (--portrayal-catalogue or --s64-zip)".to_string(),
                ..Default::default()
            },
        );
    };

    let symbol_id = inputs
        .selected_symbol_id
        .map(str::to_string)
        .or_else(|| {
            bundle
                .catalogue
                .manifest
                .symbols
                .first()
                .map(|s| s.id.clone())
        });

    let Some(symbol_id) = symbol_id else {
        return PortrayalFrame::new(
            theme,
            PortrayalLayers {
                caption: "Symbol gallery — catalogue has no symbols".to_string(),
                ..Default::default()
            },
        );
    };

    let mut symbols = Vec::new();
    let caption = match symbol_render::rasterize_symbol(
        bundle,
        &symbol_id,
        inputs.display_mode,
        192,
    ) {
        Ok(sprite) => {
            let cx = UI_CHART_VIEWBOX_WIDTH_PX * 0.5 - sprite.width_px as f32 * 0.5;
            let cy = UI_CHART_VIEWBOX_HEIGHT_PX * 0.5 - sprite.height_px as f32 * 0.5;
            symbols.push(SymbolSprite {
                symbol_id: symbol_id.clone(),
                x: cx,
                y: cy,
                width_px: sprite.width_px,
                height_px: sprite.height_px,
                rgba: sprite.rgba,
            });
            format!(
                "Symbol {symbol_id} — {}×{} px, mode {}",
                sprite.width_px,
                sprite.height_px,
                inputs.display_mode.palette_name()
            )
        }
        Err(e) => format!("Symbol {symbol_id} — render error: {e}"),
    };

    PortrayalFrame::new(
        theme,
        PortrayalLayers {
            symbols,
            caption,
            ..Default::default()
        },
    )
}

#[cfg(not(feature = "symbols"))]
pub fn build_symbol_gallery_frame(inputs: &PortrayalInputs<'_>) -> PortrayalFrame {
    let theme = ChartTheme::resolve(inputs.display_mode, active_palette(inputs));
    PortrayalFrame::new(
        theme,
        PortrayalLayers {
            caption: "Symbol gallery — build with --features symbols".to_string(),
            ..Default::default()
        },
    )
}

/// Build the frame for the active gallery scene.
pub fn build_frame(inputs: &PortrayalInputs<'_>, scene: ViewerScene) -> PortrayalFrame {
    match scene {
        ViewerScene::Chart => build_chart_frame(inputs),
        ViewerScene::FeatureGraph => build_feature_graph_frame(inputs),
        ViewerScene::SymbolGallery => build_symbol_gallery_frame(inputs),
        ViewerScene::ThemeSwatches => build_theme_swatches_frame(inputs),
    }
}
