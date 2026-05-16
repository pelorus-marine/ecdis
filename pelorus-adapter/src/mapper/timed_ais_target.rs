use crate::AisVesselReport;

use super::StreamInstant;

/// AIS report plus optional observation time.
#[derive(Debug, Clone, PartialEq)]
pub struct TimedAisTarget {
    pub report: AisVesselReport,
    pub observed_at: Option<StreamInstant>,
}
