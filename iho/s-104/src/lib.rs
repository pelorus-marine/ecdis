//! IHO **S-104** Physical Environment (weather / ocean as standardized hydrographic datasets) — stub.
//!
//! Used alongside ENC for **meteorological and oceanographic** overlays on ECDIS-class displays.
//!
//! # Status
//!
//! **Stub:** no product decoding yet.

#![forbid(unsafe_code)]

mod physical_environment_stub;

pub use physical_environment_stub::PhysicalEnvironmentStub;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stub() {
        let _ = PhysicalEnvironmentStub;
    }
}
