//! IHO **S-104** Physical Environment (weather / ocean as standardized hydrographic datasets) — stub.
//!
//! Used alongside ENC for **meteorological and oceanographic** overlays on ECDIS-class displays.
//!
//! # Status
//!
//! **Stub:** no product decoding yet.

#![forbid(unsafe_code)]

/// Placeholder until S-104 physical environment types are modeled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PhysicalEnvironmentStub;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stub() {
        let _ = PhysicalEnvironmentStub;
    }
}
