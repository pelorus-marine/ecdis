//! IHO **S-100** Universal Hydrographic Data Model — Rust representation (stub).
//!
//! Intended role: shared constructs used across **S-100 product specifications** (metadata,
//! general feature model alignment, common enumerations) so product crates (`s-101`, `s-102`, …)
//! do not duplicate framework-level types.
//!
//! # Status
//!
//! **Stub:** no normative schema binding yet. See [ARCHITECTURE.md](https://github.com/pelorus-marine/ecdis/blob/main/s-100/ARCHITECTURE.md) in the repository for scope.

#![forbid(unsafe_code)]

/// Marker type until S-100 framework types are modeled from the IHO edition in use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FrameworkStub;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stub() {
        let _ = FrameworkStub;
    }
}
