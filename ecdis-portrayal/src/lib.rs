//! **Portrayal pipeline** façade for S-101-backed ECDIS stacks.
//!
//! Builds UI-agnostic [`PortrayalFrame`] output (geometry, theme, symbols). For a developer
//! Slint gallery, see the separate `ecdis-portrayal-viewer` crate in this workspace.

#![forbid(unsafe_code)]

mod catalogue_backed;
mod catalogue_loader;
mod chart_theme;
mod chart_viewport;
mod cpu_outline;
mod display_mode;
mod feature_graph_frame;
mod frame;
mod portrayal;

#[cfg(feature = "symbols")]
mod symbol_render;

pub use catalogue_backed::{CatalogueBackedPortrayal, FeaturePortrayalDraft, PortrayalSetupError};
pub use catalogue_loader::open_portrayal_catalogue_zip;
#[cfg(feature = "s64")]
pub use catalogue_loader::{
    open_s101_feature_catalogue_from_s64_zip, open_s101_portrayal_from_s64_zip,
};
pub use chart_theme::{ChartTheme, Rgb};
pub use chart_viewport::{
    ChartViewport, ChartViewportState, UI_CHART_VIEWBOX_HEIGHT_PX, UI_CHART_VIEWBOX_WIDTH_PX,
    approx_own_ship_screen_px, demo_stub_segments_px,
};
pub use cpu_outline::CpuOutlinePortrayal;
pub use display_mode::DisplayMode;
pub use frame::{
    ColorSwatch, FilledPolygon, LineSegment, PointMarker, PortrayalFrame, PortrayalInputs,
    PortrayalLayers, SymbolSprite, ViewerScene, build_chart_frame, build_c2il_outline_frame,
    build_feature_graph_frame, build_frame, build_symbol_gallery_frame, build_theme_swatches_frame,
};
pub use portrayal::{NoPortrayal, PortrayError, PortrayalPipeline};

#[cfg(feature = "symbols")]
pub use symbol_render::{RasterizedSymbol, rasterize_symbol};
