//! **CRID**, **PTAS**, **SEGH**, **C2IL** — curve spatial records.

use crate::binary::{read_i32_le, read_u8, read_u32_le, trim_field_term};
use crate::record::identifier::RecordIdentifier;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PtasRef {
    pub rrn: u8,
    pub rrid: u32,
    pub topi: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CurveRecord {
    pub record_index: usize,
    pub id: RecordIdentifier,
    pub ptas: Vec<PtasRef>,
    pub seg_intp: Option<u8>,
    pub c2il_vertices: Vec<(i32, i32)>,
    pub tail: Vec<u8>,
}

impl CurveRecord {
    #[must_use]
    pub fn parse_multi(
        record_index: usize,
        crid: &[u8],
        ptas_slices: &[&[u8]],
        segh: Option<&[u8]>,
        c2il_slices: &[&[u8]],
    ) -> Option<Self> {
        let id = RecordIdentifier::parse(crid)?;
        let mut ptas = Vec::new();
        for s in ptas_slices {
            ptas.extend(parse_ptas(trim_field_term(s)));
        }
        let seg_intp = segh.and_then(|s| trim_field_term(s).first().copied());
        let mut c2il_concat = Vec::<u8>::new();
        for s in c2il_slices {
            c2il_concat.extend_from_slice(trim_field_term(s));
        }
        let p = c2il_concat.as_slice();
        let mut off = 0usize;
        let mut c2il_vertices = Vec::new();
        while off + 8 <= p.len() {
            let y = read_i32_le(p, &mut off)?;
            let x = read_i32_le(p, &mut off)?;
            c2il_vertices.push((y, x));
        }
        let tail = p.get(off..).unwrap_or_default().to_vec();
        Some(Self {
            record_index,
            id,
            ptas,
            seg_intp,
            c2il_vertices,
            tail,
        })
    }
}

#[must_use]
fn parse_ptas(p: &[u8]) -> Vec<PtasRef> {
    let mut out = Vec::new();
    let mut off = 0usize;
    while off + 6 <= p.len() {
        let rrn = match read_u8(p, &mut off) {
            Some(v) => v,
            None => break,
        };
        let rrid = match read_u32_le(p, &mut off) {
            Some(v) => v,
            None => break,
        };
        let topi = match read_u8(p, &mut off) {
            Some(v) => v,
            None => break,
        };
        out.push(PtasRef { rrn, rrid, topi });
    }
    out
}
