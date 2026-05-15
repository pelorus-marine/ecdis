//! IHO **S-61** — Raster Navigational Charts (RNC) (placeholder crate).
//!
//! IHO **raster** chart product (pixels). Distinct from **S-101** vector ENC and the **S-100** vector family. Reserve/implementation crate.
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
