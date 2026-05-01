/// Snapshot of **own ship** state for chart-centric applications.
///
/// Map fields from **Pelorus Core** talkers (GNSS, heading, speed log, echosounder, etc.) in the
/// integration layer outside this crate; DCID bindings are intentionally **not** hard-coded here
/// so Core schema can evolve independently.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct OwnShip {
    /// Latitude in degrees (WGS84), north positive.
    pub lat_deg: Option<f64>,
    /// Longitude in degrees (WGS84), east positive.
    pub lon_deg: Option<f64>,
    /// Course over ground, degrees true [0, 360).
    pub cog_true_deg: Option<f64>,
    /// Speed over ground, metres per second.
    pub sog_mps: Option<f64>,
    /// Heading (degrees true).
    pub heading_true_deg: Option<f64>,
    /// Under-keel or raw depth below transducer in metres (product-specific semantics upstream).
    pub depth_m: Option<f64>,
}

impl OwnShip {
    pub fn with_position(lat_deg: f64, lon_deg: f64) -> Self {
        Self {
            lat_deg: Some(lat_deg),
            lon_deg: Some(lon_deg),
            ..Default::default()
        }
    }
}
