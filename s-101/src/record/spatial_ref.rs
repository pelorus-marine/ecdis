//! **SPAS** spatial association field.

use crate::binary::{read_u8, read_u32_le, trim_field_term};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpasRef {
    pub rrn: u8,
    pub rrid: u32,
    pub ornt: u8,
    pub smin: u32,
    pub smax: u32,
    pub saui: u8,
}

impl SpasRef {
    #[must_use]
    pub fn parse(payload: &[u8]) -> Option<Self> {
        let p = trim_field_term(payload);
        if p.len() < 15 {
            return None;
        }
        let mut off = 0usize;
        Some(Self {
            rrn: read_u8(p, &mut off)?,
            rrid: read_u32_le(p, &mut off)?,
            ornt: read_u8(p, &mut off)?,
            smin: read_u32_le(p, &mut off)?,
            smax: read_u32_le(p, &mut off)?,
            saui: read_u8(p, &mut off)?,
        })
    }
}
