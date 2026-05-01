//! IHO **S-102** Bathymetric Surface — grid coverage and depth model (stub).
//!
//! S-102 provides gridded bathymetry complementary to **S-101** vector ENC data. This crate
//! will decode S-102 **BathymetricSurface** coverage instances from S-100 exchange packaging.
//!
//! # Status
//!
//! **Stub:** no grid decoding yet.

#![forbid(unsafe_code)]

/// Placeholder until bathymetric grid and metadata types are defined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BathymetricSurfaceStub;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stub() {
        let _ = BathymetricSurfaceStub;
    }
}
