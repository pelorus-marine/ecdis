use super::curve2d::Curve2D;

/// Simple polygon: one exterior ring and zero or more interior rings (holes), WGS84.
#[derive(Debug, Clone, PartialEq)]
pub struct Surface2D {
    pub exterior: Curve2D,
    pub interiors: Vec<Curve2D>,
}

impl Surface2D {
    #[must_use]
    pub fn new(exterior: Curve2D, interiors: Vec<Curve2D>) -> Self {
        Self {
            exterior,
            interiors,
        }
    }
}
