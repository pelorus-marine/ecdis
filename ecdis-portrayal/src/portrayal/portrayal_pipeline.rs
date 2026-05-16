use s_101::S101Dataset;

use super::PortrayError;

/// Backend prepares chart presentation for a given display scale denominator.
pub trait PortrayalPipeline {
    fn reset_for_chart(&mut self, chart: &S101Dataset) -> Result<(), PortrayError>;
    fn set_display_scale(&mut self, scale_denominator: u32) -> Result<(), PortrayError>;
}
