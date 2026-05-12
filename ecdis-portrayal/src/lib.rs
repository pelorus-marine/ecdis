//! **Portrayal pipeline** façade for S-101-backed ECDIS stacks.
//!
//! Full **S-100 portrayal / AML** execution is not implemented — this crate defines stable hooks
//! [`PortrayalPipeline`] so runtime/UI layers can swap GPU or CPU backends later.

#![forbid(unsafe_code)]

mod catalogue_backed;
mod chart_viewport;
mod cpu_outline;

use std::fmt;

pub use catalogue_backed::{CatalogueBackedPortrayal, FeaturePortrayalDraft, PortrayalSetupError};
pub use chart_viewport::{
    ChartViewport, ChartViewportState, UI_CHART_VIEWBOX_HEIGHT_PX, UI_CHART_VIEWBOX_WIDTH_PX,
    approx_own_ship_screen_px, demo_stub_segments_px,
};
pub use cpu_outline::CpuOutlinePortrayal;
use s_101::S101Dataset;

/// Errors from portrayal preparation (stub).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortrayError {
    UnsupportedScale,
}

impl fmt::Display for PortrayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedScale => write!(f, "unsupported display scale"),
        }
    }
}

impl std::error::Error for PortrayError {}

/// Backend prepares chart presentation for a given display scale denominator.
pub trait PortrayalPipeline {
    fn reset_for_chart(&mut self, chart: &S101Dataset) -> Result<(), PortrayError>;
    fn set_display_scale(&mut self, scale_denominator: u32) -> Result<(), PortrayError>;
}

/// No-op backend — validates trait wiring without AML assets.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoPortrayal;

impl PortrayalPipeline for NoPortrayal {
    fn reset_for_chart(&mut self, _chart: &S101Dataset) -> Result<(), PortrayError> {
        Ok(())
    }

    fn set_display_scale(&mut self, _scale_denominator: u32) -> Result<(), PortrayError> {
        Ok(())
    }
}
