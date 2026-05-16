//! IHO **S-111** Surface Currents — stub.
//!
//! Provides **current field** datasets for ECDIS overlay and voyage planning when implemented.

#![forbid(unsafe_code)]

mod surface_currents_stub;

pub use surface_currents_stub::SurfaceCurrentsStub;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stub() {
        let _ = SurfaceCurrentsStub;
    }
}
