use super::{
    curve2d::Curve2D, multi_point2d::MultiPoint2D, point2d::Point2D, surface2d::Surface2D,
};

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
