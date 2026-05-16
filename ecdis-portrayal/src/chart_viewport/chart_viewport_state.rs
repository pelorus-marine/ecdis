/// Mariner chart framing (degrees + conventional scale denominator).
#[derive(Debug, Clone, PartialEq)]
pub struct ChartViewportState {
    pub center_lon_deg: f64,
    pub center_lat_deg: f64,
    pub scale_denominator: u32,
}

impl Default for ChartViewportState {
    fn default() -> Self {
        Self {
            center_lon_deg: 2.0,
            center_lat_deg: 51.0,
            scale_denominator: 22_000,
        }
    }
}
