use super::point2d::Point2D;

/// Open polyline as straight segments between consecutive vertices (WGS84).
#[derive(Debug, Clone, PartialEq)]
pub struct Curve2D {
    pub vertices: Vec<Point2D>,
}

impl Curve2D {
    #[must_use]
    pub fn new(vertices: Vec<Point2D>) -> Self {
        Self { vertices }
    }
}
