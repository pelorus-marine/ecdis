//! **MRID** + **C3IL** — 3-D / composite point chains (S-64 DualFuel uses `RRNM == 115`).

use crate::binary::{read_i32_le, trim_field_term};
use crate::record::identifier::RecordIdentifier;

#[derive(Debug, Clone, PartialEq)]
pub struct MridRecord {
    pub record_index: usize,
    pub id: RecordIdentifier,
    /// Integer grid `(Y, X)` pairs decoded from each **C3IL** triplet (Z discarded for WGS84 probe).
    pub yx_pairs: Vec<(i32, i32)>,
    pub tail: Vec<u8>,
}

impl MridRecord {
    #[must_use]
    pub fn parse(record_index: usize, mrid: &[u8], c3il: &[u8]) -> Option<Self> {
        let id = RecordIdentifier::parse(mrid)?;
        let p = trim_field_term(c3il);
        let mut off = 0usize;
        let mut yx_pairs = Vec::new();
        while off + 12 <= p.len() {
            let y = read_i32_le(p, &mut off)?;
            let x = read_i32_le(p, &mut off)?;
            let _z = read_i32_le(p, &mut off)?;
            yx_pairs.push((y, x));
        }
        let tail = p.get(off..).unwrap_or_default().to_vec();
        Some(Self {
            record_index,
            id,
            yx_pairs,
            tail,
        })
    }
}
