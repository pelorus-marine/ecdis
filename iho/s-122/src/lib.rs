//! IHO **S-122** — Marine protected areas (placeholder crate).
//!
//! S-100 **marine protected areas** product (check current IHO edition for feature catalogue).
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
