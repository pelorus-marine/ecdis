use s_101::S101Dataset;

use crate::portrayal::{PortrayError, PortrayalPipeline};

use super::ChartViewportState;

/// Couples [`ChartViewportState`] with a [`PortrayalPipeline`] implementation.
pub struct ChartViewport<P: PortrayalPipeline> {
    pub state: ChartViewportState,
    portrayal: P,
}

impl<P: PortrayalPipeline> ChartViewport<P> {
    #[must_use]
    pub fn new(portrayal: P) -> Self {
        Self {
            state: ChartViewportState::default(),
            portrayal,
        }
    }

    #[must_use]
    pub fn portrayal_ref(&self) -> &P {
        &self.portrayal
    }

    pub fn portrayal_mut(&mut self) -> &mut P {
        &mut self.portrayal
    }

    pub fn reset_chart(&mut self, chart: &S101Dataset) -> Result<(), PortrayError> {
        self.portrayal.reset_for_chart(chart)?;
        self.portrayal.set_display_scale(self.state.scale_denominator)?;
        Ok(())
    }

    pub fn set_scale_from_mariner(
        &mut self,
        _chart: &S101Dataset,
        scale_denom: u32,
    ) -> Result<(), PortrayError> {
        self.portrayal.set_display_scale(scale_denom)?;
        self.state.scale_denominator = scale_denom;
        Ok(())
    }

    pub fn nudge_scale(
        &mut self,
        chart: &S101Dataset,
        factor_mul: f64,
    ) -> Result<(), PortrayError> {
        let raw = (f64::from(self.state.scale_denominator) * factor_mul).round();
        let next = (raw.clamp(500.0, 50_000_000.0)) as u32;
        self.set_scale_from_mariner(chart, next)
    }

    pub fn pan_deg(&mut self, dlon: f64, dlat: f64) {
        self.state.center_lon_deg += dlon;
        self.state.center_lat_deg += dlat;
    }
}
