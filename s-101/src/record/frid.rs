//! **FRID** feature record identifier.

use crate::binary::{read_u8, read_u16_le, read_u32_le, trim_field_term};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FridHeader {
    pub rcnm: u8,
    pub rcid: u32,
    pub nftc: u16,
    pub rver: u8,
    pub ruin: u8,
}

impl FridHeader {
    #[must_use]
    pub fn parse(payload: &[u8]) -> Option<Self> {
        let p = trim_field_term(payload);
        if p.len() < 9 {
            return None;
        }
        let mut off = 0usize;
        Some(Self {
            rcnm: read_u8(p, &mut off)?,
            rcid: read_u32_le(p, &mut off)?,
            nftc: read_u16_le(p, &mut off)?,
            rver: read_u8(p, &mut off)?,
            ruin: read_u8(p, &mut off)?,
        })
    }
}
