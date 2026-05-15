//! IHO **S-127** Marine Protected Areas — stub.
//!
//! Structured **MPA** limits for chart and ECDIS awareness layers.

#![forbid(unsafe_code)]

/// Placeholder until S-127 MPA features are modeled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MarineProtectedAreasStub;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stub() {
        let _ = MarineProtectedAreasStub;
    }
}
