//! IHO **S-131** — Marine harbour infrastructure (placeholder crate).
//!
//! S-100 product for harbour infrastructure.
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
