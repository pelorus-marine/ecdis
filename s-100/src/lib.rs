//! IHO **S-100** Universal Hydrographic Data Model — Rust representation.
//!
//! Shared **geometry** (WGS84) and **identifiers** used by product crates (`s-101`, …) and
//! presentation layers ([`ecdis_portrayal`](https://crates.io/crates/ecdis-portrayal)).
//!
//! # Status
//!
//! Framework modelling is still incremental; see [ARCHITECTURE.md](ARCHITECTURE.md).

#![forbid(unsafe_code)]

pub mod feature_id;
pub mod geometry;

pub use feature_id::FeatureObjectId;
pub use geometry::{Curve2D, Geometry, MultiPoint2D, Point2D, Surface2D};

/// Marker type retained for early workspace wiring; prefer concrete types in this crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FrameworkStub;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometry_point_not_empty() {
        let g = Geometry::Point(Point2D::new(1.0, 2.0));
        assert!(!g.is_empty());
    }

    #[test]
    fn framework_stub() {
        let _ = FrameworkStub;
    }
}
