//! IHO **S-97** — Guidelines for S-100 product specifications (placeholder crate).
//!
//! IHO guidance for authors of S-100-based product specifications; this crate is a placeholder for future tooling or type stubs aligned with [S-97](https://iho.int/).
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
