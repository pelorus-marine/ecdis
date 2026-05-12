//! WGS84 geometry primitives shared across S-100 product decoders and portrayal.

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

/// Unordered multi-point geometry.
#[derive(Debug, Clone, PartialEq)]
pub struct MultiPoint2D(pub Vec<Point2D>);

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

/// Geometry attached to a feature after CRS resolution.
#[derive(Debug, Clone, PartialEq)]
pub enum Geometry {
    Point(Point2D),
    MultiPoint(MultiPoint2D),
    Curve(Curve2D),
    Surface(Surface2D),
}

impl Geometry {
    /// `true` when this variant carries at least one drawable vertex.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Point(_) => false,
            Self::MultiPoint(m) => m.0.is_empty(),
            Self::Curve(c) => c.vertices.len() < 2,
            Self::Surface(s) => s.exterior.vertices.len() < 2,
        }
    }
}
