//! IHO **S-102** Bathymetric Surface — grid coverage and depth model (stub).
//!
//! S-102 provides gridded bathymetry complementary to **S-101** vector ENC data. This crate
//! will decode S-102 **BathymetricSurface** coverage instances from S-100 exchange packaging.
//!
//! # Status
//!
//! **Stub:** no grid decoding yet.

#![forbid(unsafe_code)]

mod bathymetric_surface_stub;

pub use bathymetric_surface_stub::BathymetricSurfaceStub;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stub() {
        let _ = BathymetricSurfaceStub;
    }
}
