//! IHO **S-129** Under Keel Clearance Management — stub.
//!
//! Encodes **UKC**-related data for ECDIS decision support when integrated with **S-102**
//! bathymetry and ENC.

#![forbid(unsafe_code)]

mod under_keel_clearance_stub;

pub use under_keel_clearance_stub::UnderKeelClearanceStub;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stub() {
        let _ = UnderKeelClearanceStub;
    }
}
