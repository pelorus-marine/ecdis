//! IHO **S-98** — Data product interoperability (placeholder crate).
//!
//! Companion to **S-100** for **how multiple S-100 products interoperate on ECDIS and navigation systems** (display coordination, clutter, conflicts)—not a chart cell format. See [IHO S-98](https://iho.int/).
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
