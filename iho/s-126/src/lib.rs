//! IHO **S-126** — Marine physical environment (placeholder crate).
//!
//! S-100 product for **marine physical environment** (distinct from S-104 water-level focus in current IHO naming—verify edition).
//!
//! # Status
//! **v0.0.1** reserves the crates.io name; implementation may follow in later versions.

#![forbid(unsafe_code)]

mod development_stub;

pub use development_stub::DevelopmentStub;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smoke() {
        let _ = DevelopmentStub;
    }
}
