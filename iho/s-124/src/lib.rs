//! IHO **S-124** Navigational Warnings in the S-100 family — stub.
//!
//! Supports **NAVAREA / coastal warning** style content as structured S-100 datasets for ECDIS.

#![forbid(unsafe_code)]

/// Placeholder until S-124 warning records and geometry are modeled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct NavigationalWarningsStub;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stub() {
        let _ = NavigationalWarningsStub;
    }
}
