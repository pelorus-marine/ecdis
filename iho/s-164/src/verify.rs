//! SHA-256 integrity verification for downloaded corpus archives.

use sha2::{Digest, Sha256};

use crate::{S164Error, S164Result};

/// Compute the lowercase hex SHA-256 of a byte slice.
#[must_use]
pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    let mut hex = String::with_capacity(64);
    for byte in digest {
        use std::fmt::Write;
        let _ = write!(&mut hex, "{byte:02x}");
    }
    hex
}

/// Verify `bytes` hashes to `expected` (lowercase hex, 64 chars).
///
/// `what` is a label used in the error message (e.g. a URL or cache path).
pub fn verify_sha256(bytes: &[u8], expected: &str, what: &str) -> S164Result<()> {
    let actual = sha256_hex(bytes);
    if actual.eq_ignore_ascii_case(expected) {
        Ok(())
    } else {
        Err(S164Error::Sha256Mismatch {
            what: what.to_string(),
            expected: expected.to_string(),
            actual,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_of_known_input() {
        // sha256("") == e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn verify_accepts_match_case_insensitively() {
        let bytes = b"hello";
        let h = sha256_hex(bytes);
        verify_sha256(bytes, &h.to_uppercase(), "test").unwrap();
    }

    #[test]
    fn verify_rejects_mismatch() {
        let err = verify_sha256(b"hello", &"0".repeat(64), "test").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("SHA-256 mismatch"), "{msg}");
    }
}
