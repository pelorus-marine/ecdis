//! **FOID** — feature object identifier triple.

use s_100::FeatureObjectId;

use crate::binary::{read_u16_le, read_u32_le, trim_field_term};

#[must_use]
pub fn parse_foid(payload: &[u8]) -> Option<FeatureObjectId> {
    let p = trim_field_term(payload);
    if p.len() < 8 {
        return None;
    }
    let mut off = 0usize;
    let agency = read_u16_le(p, &mut off)?;
    let fidn = read_u32_le(p, &mut off)?;
    let fids = read_u16_le(p, &mut off)?;
    Some(FeatureObjectId::new(agency, fidn, fids))
}
