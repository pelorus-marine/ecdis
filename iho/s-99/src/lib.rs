//! IHO **S-99** — GI registry operational procedures (placeholder crate).
//!
//! IHO procedures for the geospatial information registry; placeholder crate for future automation or metadata types.
//!
//! # Status
//! **v0.0.1** reserves the crates.io name; implementation may follow in later versions.

#![forbid(unsafe_code)]

/// Reserved until normative schema bindings are added.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DevelopmentStub;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smoke() {
        let _ = DevelopmentStub;
    }
}
