//! **CCID** + **CUCO** — composite curve records.

use crate::binary::{read_u8, read_u32_le, trim_field_term};
use crate::record::cuco_ref::CucoRef;
use crate::record::identifier::RecordIdentifier;

#[derive(Debug, Clone, PartialEq)]
pub struct CompositeCurveRecord {
    pub record_index: usize,
    pub id: RecordIdentifier,
    pub components: Vec<CucoRef>,
    pub tail: Vec<u8>,
}

impl CompositeCurveRecord {
    #[must_use]
    pub fn parse(record_index: usize, ccid: &[u8], cuco_fields: &[&[u8]]) -> Option<Self> {
        let id = RecordIdentifier::parse(ccid)?;
        let mut components = Vec::new();
        for c in cuco_fields {
            let p = trim_field_term(c);
            if p.len() < 6 {
                continue;
            }
            let mut off = 0usize;
            if let (Some(rrn), Some(rrid), Some(orientation)) = (
                read_u8(p, &mut off),
                read_u32_le(p, &mut off),
                read_u8(p, &mut off),
            ) {
                components.push(CucoRef {
                    rrn,
                    rrid,
                    orientation,
                });
            }
        }
        Some(Self {
            record_index,
            id,
            components,
            tail: Vec::new(),
        })
    }
}
