//! IHO **S-111** Surface Currents — stub.
//!
//! Provides **current field** datasets for ECDIS overlay and voyage planning when implemented.

#![forbid(unsafe_code)]

/// Placeholder until S-111 vector / grid current model is implemented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SurfaceCurrentsStub;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stub() {
        let _ = SurfaceCurrentsStub;
    }
}
