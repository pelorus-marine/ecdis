//! IHO **S-99** — GI registry operational procedures (placeholder crate).
//!
//! IHO procedures for the geospatial information registry; placeholder crate for future automation or metadata types.
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
