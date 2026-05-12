//! **ATTR** field — attribute tuples (`ATIX`, `PAIX`, `ATIN`, `ATVL`).

use crate::binary::{read_u16_le, trim_field_term};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawAttributeTuple {
    pub atix: u16,
    pub paix: u16,
    pub atin: u16,
    pub atvl: Vec<u8>,
}

/// Parse **ATTR** payload as repeated `(u16,u16,u16, bytes until 0x1F)` tuples.
#[must_use]
pub fn parse_attr_tuples(payload: &[u8]) -> Vec<RawAttributeTuple> {
    let p = trim_field_term(payload);
    let mut off = 0usize;
    let mut out = Vec::new();
    while off + 6 <= p.len() {
        let atix = read_u16_le(p, &mut off).unwrap_or(0);
        let paix = read_u16_le(p, &mut off).unwrap_or(0);
        let atin = read_u16_le(p, &mut off).unwrap_or(0);
        let start = off;
        while off < p.len() && p[off] != 0x1f {
            off += 1;
        }
        let atvl = p[start..off].to_vec();
        if off < p.len() {
            off += 1;
        }
        out.push(RawAttributeTuple {
            atix,
            paix,
            atin,
            atvl,
        });
    }
    out
}
