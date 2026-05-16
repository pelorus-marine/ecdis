use s_101::S101Dataset;

use super::{PortrayError, PortrayalPipeline};

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
