//! Record identifier header shared by spatial records (`PRID`, `CRID`, …).

use crate::binary::{read_u8, read_u32_le, trim_field_term};

/// Parsed **RCNM / RCID / RVER / RUIN** header (S-100 interchange pattern used in S-64 v1.2.0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordIdentifier {
    pub rcnm: u8,
    pub rcid: u32,
    pub rver: u8,
    pub ruin: u8,
}

impl RecordIdentifier {
    /// Decode from a trimmed field payload (trailing `0x1E` removed).
    #[must_use]
    pub fn parse(payload: &[u8]) -> Option<Self> {
        let p = trim_field_term(payload);
        if p.len() < 6 {
            return None;
        }
        let mut off = 0usize;
        let rcnm = read_u8(p, &mut off)?;
        let rcid = read_u32_le(p, &mut off)?;
        let rver = read_u8(p, &mut off)?;
        let ruin = read_u8(p, &mut off)?;
        Some(Self {
            rcnm,
            rcid,
            rver,
            ruin,
        })
    }
}
