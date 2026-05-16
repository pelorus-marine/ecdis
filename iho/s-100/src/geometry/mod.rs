//! WGS84 geometry primitives shared across S-100 product decoders and portrayal.
//!
//! Each public type lives in its own file under `geometry/`; this `mod.rs` is just
//! the namespace assembly point.

mod curve2d;
mod geometry;
mod multi_point2d;
mod point2d;
mod surface2d;

pub use curve2d::Curve2D;
pub use geometry::Geometry;
pub use multi_point2d::MultiPoint2D;
pub use point2d::Point2D;
pub use surface2d::Surface2D;
