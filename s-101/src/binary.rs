//! Low-level binary helpers for **S-101** ISO 8211 field payloads.

/// Strip ISO 8211 **field terminator** (`0x1E`) if present at the end.
#[must_use]
pub fn trim_field_term(mut p: &[u8]) -> &[u8] {
    if p.last().copied() == Some(0x1e) {
        p = &p[..p.len() - 1];
    }
    p
}

#[must_use]
pub fn read_u16_le(p: &[u8], off: &mut usize) -> Option<u16> {
    let b = p.get(*off..*off + 2)?;
    *off += 2;
    Some(u16::from_le_bytes([b[0], b[1]]))
}

#[must_use]
pub fn read_u32_le(p: &[u8], off: &mut usize) -> Option<u32> {
    let b = p.get(*off..*off + 4)?;
    *off += 4;
    Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

#[must_use]
pub fn read_i32_le(p: &[u8], off: &mut usize) -> Option<i32> {
    let b = p.get(*off..*off + 4)?;
    *off += 4;
    Some(i32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

#[must_use]
pub fn read_u8(p: &[u8], off: &mut usize) -> Option<u8> {
    let v = *p.get(*off)?;
    *off += 1;
    Some(v)
}

/// Parse **FTCS** (feature type catalogue) payload: repeated `UTF-8 name` + `0x1F` + `u16` LE code.
#[must_use]
pub fn parse_ftcs(payload: &[u8]) -> Vec<(String, u16)> {
    let p = trim_field_term(payload);
    let mut i = 0usize;
    let mut out = Vec::new();
    while i < p.len() {
        let start = i;
        while i < p.len() && p[i] != 0x1f {
            i += 1;
        }
        let name = String::from_utf8_lossy(&p[start..i]).into_owned();
        if i < p.len() {
            i += 1;
        }
        if i + 2 <= p.len() {
            let code = u16::from_le_bytes([p[i], p[i + 1]]);
            i += 2;
            out.push((name, code));
        } else if !name.is_empty() {
            out.push((name, 0));
        }
    }
    out
}
