//! IHO **S-131** — Marine harbour infrastructure (placeholder crate).
//!
//! S-100 product for harbour infrastructure.
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
