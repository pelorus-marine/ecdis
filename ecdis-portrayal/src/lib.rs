//! **Portrayal pipeline** façade for S-101-backed ECDIS stacks.
//!
//! Full **S-100 portrayal / AML** execution is not implemented — this crate defines stable hooks
//! [`PortrayalPipeline`] so runtime/UI layers can swap GPU or CPU backends later.
//!
//! Each public type lives in its own file; module `mod.rs` files are namespace assembly points.

#![forbid(unsafe_code)]

mod catalogue_backed;
mod chart_viewport;
mod cpu_outline;
mod portrayal;

pub use catalogue_backed::{CatalogueBackedPortrayal, FeaturePortrayalDraft, PortrayalSetupError};
pub use chart_viewport::{
    ChartViewport, ChartViewportState, UI_CHART_VIEWBOX_HEIGHT_PX, UI_CHART_VIEWBOX_WIDTH_PX,
    approx_own_ship_screen_px, demo_stub_segments_px,
};
pub use cpu_outline::CpuOutlinePortrayal;
pub use portrayal::{NoPortrayal, PortrayError, PortrayalPipeline};
