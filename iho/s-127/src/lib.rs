//! IHO **S-127** Marine Protected Areas — stub.
//!
//! Structured **MPA** limits for chart and ECDIS awareness layers.

#![forbid(unsafe_code)]

mod marine_protected_areas_stub;

pub use marine_protected_areas_stub::MarineProtectedAreasStub;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stub() {
        let _ = MarineProtectedAreasStub;
    }
}
