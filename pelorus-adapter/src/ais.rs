/// Minimal **AIS** target report for overlay planning (not a full VDM parser).
///
/// Populate from Pelorus Core or a dedicated AIS decoder; this type only carries what chart /
/// alert logic commonly needs alongside [`super::OwnShip`](crate::OwnShip).
#[derive(Debug, Clone, PartialEq)]
pub struct AisVesselReport {
    pub mmsi: u32,
    pub lat_deg: Option<f64>,
    pub lon_deg: Option<f64>,
    pub sog_mps: Option<f64>,
    pub cog_true_deg: Option<f64>,
    pub heading_true_deg: Option<f64>,
}

impl AisVesselReport {
    pub fn new(mmsi: u32) -> Self {
        Self {
            mmsi,
            lat_deg: None,
            lon_deg: None,
            sog_mps: None,
            cog_true_deg: None,
            heading_true_deg: None,
        }
    }
}
