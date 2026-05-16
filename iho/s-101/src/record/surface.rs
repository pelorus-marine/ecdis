//! **SRID** + **RIAS** — surface spatial records.

use crate::binary::{read_u8, read_u32_le, trim_field_term};
use crate::record::identifier::RecordIdentifier;
use crate::record::rias_ref::RiasRef;

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceRecord {
    pub record_index: usize,
    pub id: RecordIdentifier,
    pub rings: Vec<RiasRef>,
    pub tail: Vec<u8>,
}

impl SurfaceRecord {
    #[must_use]
    pub fn parse(record_index: usize, srid: &[u8], rias_fields: &[&[u8]]) -> Option<Self> {
        let id = RecordIdentifier::parse(srid)?;
        let mut rings = Vec::new();
        for rf in rias_fields {
            if let Some(r) = parse_one_rias(rf) {
                rings.push(r);
            }
        }
        Some(Self {
            record_index,
            id,
            rings,
            tail: Vec::new(),
        })
    }
}

fn parse_one_rias(payload: &[u8]) -> Option<RiasRef> {
    let p = trim_field_term(payload);
    if p.len() < 8 {
        return None;
    }
    let mut off = 0usize;
    Some(RiasRef {
        rrn: read_u8(p, &mut off)?,
        rrid: read_u32_le(p, &mut off)?,
        ornt: read_u8(p, &mut off)?,
        usag: read_u8(p, &mut off)?,
        raui: read_u8(p, &mut off)?,
    })
}
