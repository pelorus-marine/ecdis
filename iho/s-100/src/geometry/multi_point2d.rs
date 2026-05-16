use super::point2d::Point2D;

/// Unordered multi-point geometry.
#[derive(Debug, Clone, PartialEq)]
pub struct MultiPoint2D(pub Vec<Point2D>);
