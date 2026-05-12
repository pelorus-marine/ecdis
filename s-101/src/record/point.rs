//! **PRID** + **C2IT** — point spatial records.

use crate::binary::{read_i32_le, trim_field_term};
use crate::record::identifier::RecordIdentifier;

#[derive(Debug, Clone, PartialEq)]
pub struct PointRecord {
    pub record_index: usize,
    pub id: RecordIdentifier,
    pub ycoo: i32,
    pub xcoo: i32,
    pub tail: Vec<u8>,
}

impl PointRecord {
    #[must_use]
    pub fn parse(record_index: usize, prid: &[u8], c2it: &[u8]) -> Option<Self> {
        let id = RecordIdentifier::parse(prid)?;
        let p = trim_field_term(c2it);
        if p.len() < 8 {
            return None;
        }
        let mut off = 0usize;
        let ycoo = read_i32_le(p, &mut off)?;
        let xcoo = read_i32_le(p, &mut off)?;
        let tail = p.get(off..).unwrap_or_default().to_vec();
        Some(Self {
            record_index,
            id,
            ycoo,
            xcoo,
            tail,
        })
    }
}
