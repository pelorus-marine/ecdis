//! IHO **S-124** Navigational Warnings in the S-100 family — stub.
//!
//! Supports **NAVAREA / coastal warning** style content as structured S-100 datasets for ECDIS.

#![forbid(unsafe_code)]

mod navigational_warnings_stub;

pub use navigational_warnings_stub::NavigationalWarningsStub;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stub() {
        let _ = NavigationalWarningsStub;
    }
}
