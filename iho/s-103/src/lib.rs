//! IHO **S-103** *Sub-surface Navigation* — S-100 product (stub).
//!
//! Complements **S-101** ENC near coastal and confined operations; decoding will follow the same
//! **ISO 8211** → product pattern as other S-100 family crates.
//!
//! # Status
//!
//! **Stub:** no product decoder yet.

#![forbid(unsafe_code)]

mod subsurface_navigation_stub;

pub use subsurface_navigation_stub::SubsurfaceNavigationStub;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stub() {
        let _ = SubsurfaceNavigationStub;
    }
}
