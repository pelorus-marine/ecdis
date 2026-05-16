/// Single geographic position in **degrees** (latitude north, longitude east).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub lat_deg: f64,
    pub lon_deg: f64,
}

impl Point2D {
    #[must_use]
    pub const fn new(lat_deg: f64, lon_deg: f64) -> Self {
        Self { lat_deg, lon_deg }
    }
}
