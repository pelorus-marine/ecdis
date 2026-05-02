//! **S-101 / S-100** geometry helpers — first slice decodes **DSSI** CRS knobs and **C2IL** integer pairs.
//!
//! Coordinate conversion follows S-101 ENC integer CRS usage: `lon = DCOX + XCOO / CMFX`,
//! `lat = DCOY + YCOO / CMFY` (see IHO S-101 product specification / S-100 Part 10a).

use crate::S101Dataset;
use crate::decode::record_field_payload;

/// CRS parameters from the **DSSI** field on dataset record 0 (empirical layout for current test cells).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntegerCrsParameters {
    pub dco_x: f64,
    pub dco_y: f64,
    pub cmf_x: u32,
    pub cmf_y: u32,
}

impl Default for IntegerCrsParameters {
    fn default() -> Self {
        Self {
            dco_x: 0.0,
            dco_y: 0.0,
            cmf_x: 10_000_000,
            cmf_y: 10_000_000,
        }
    }
}

/// Strip ISO 8211 **unit terminator** (`0x1E`) suffix if present.
#[must_use]
pub fn trim_iso8211_unit_term(mut payload: &[u8]) -> &[u8] {
    if let Some(last) = payload.last().copied()
        && last == 0x1E
    {
        payload = &payload[..payload.len() - 1];
    }
    payload
}

/// Parse **DSSI** payload: three IEEE754 LE doubles (DCOX, DCOY, DCOZ) followed by CMFX/CMFY as `u32` LE.
///
/// Returns [`None`] if `payload` is too short or CMF values are zero.
#[must_use]
pub fn parse_dssi_integer_crs(payload: &[u8]) -> Option<IntegerCrsParameters> {
    let p = trim_iso8211_unit_term(payload);
    if p.len() < 32 {
        return None;
    }
    let dco_x = f64::from_le_bytes(p[0..8].try_into().ok()?);
    let dco_y = f64::from_le_bytes(p[8..16].try_into().ok()?);
    let cmf_x = u32::from_le_bytes(p[24..28].try_into().ok()?);
    let cmf_y = u32::from_le_bytes(p[28..32].try_into().ok()?);
    if cmf_x == 0 || cmf_y == 0 {
        return None;
    }
    Some(IntegerCrsParameters {
        dco_x,
        dco_y,
        cmf_x,
        cmf_y,
    })
}

/// Decode **C2IL** payload into integer `(YCOO, XCOO)` tuples (pairs of `i32` LE).
#[must_use]
pub fn decode_c2il_integer_pairs(payload: &[u8]) -> Vec<(i32, i32)> {
    let p = trim_iso8211_unit_term(payload);
    let mut out = Vec::with_capacity(p.len() / 8);
    let mut off = 0usize;
    while off + 8 <= p.len() {
        let y = i32::from_le_bytes(p[off..off + 4].try_into().unwrap());
        let x = i32::from_le_bytes(p[off + 4..off + 8].try_into().unwrap());
        out.push((y, x));
        off += 8;
    }
    out
}

#[must_use]
fn ycoo_xcoo_to_wgs84_deg(crs: &IntegerCrsParameters, y: i32, x: i32) -> (f64, f64) {
    let lat = crs.dco_y + f64::from(y) / f64::from(crs.cmf_y);
    let lon = crs.dco_x + f64::from(x) / f64::from(crs.cmf_x);
    (lat, lon)
}

/// Extract **C2IL** polylines as WGS84 `(lat°, lon°)` chains (one chain per C2IL field instance).
///
/// Stops after `max_points` decoded vertices to bound UI work.
#[must_use]
pub fn extract_c2il_polylines_wgs84(
    dataset: &S101Dataset,
    max_points: usize,
) -> (IntegerCrsParameters, Vec<Vec<(f64, f64)>>) {
    let crs = dataset
        .iso8211()
        .data_records()
        .first()
        .and_then(|r| record_field_payload(r, "DSSI"))
        .and_then(parse_dssi_integer_crs)
        .unwrap_or_default();

    let mut chains: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut budget = max_points;

    for rec in dataset.iso8211().data_records() {
        if budget == 0 {
            break;
        }
        for (_, df) in
            rec.field_tags.iter().zip(rec.data_fields.iter()).filter(|(t, _)| *t == "C2IL")
        {
            if budget == 0 {
                break;
            }
            let pairs = decode_c2il_integer_pairs(df.user_data());
            if pairs.len() < 2 {
                continue;
            }
            let mut chain = Vec::with_capacity(pairs.len());
            for (y, x) in pairs {
                if budget == 0 {
                    break;
                }
                chain.push(ycoo_xcoo_to_wgs84_deg(&crs, y, x));
                budget -= 1;
            }
            if chain.len() >= 2 {
                chains.push(chain);
            }
        }
    }

    (crs, chains)
}

impl S101Dataset {
    /// Parsed integer CRS parameters from the first record **DSSI** field, if decodable.
    #[must_use]
    pub fn integer_crs_parameters(&self) -> Option<IntegerCrsParameters> {
        let first = self.iso8211().data_records().first()?;
        let payload = record_field_payload(first, "DSSI")?;
        parse_dssi_integer_crs(payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_sample_dssi_layout() {
        let mut bytes = [0u8; 32];
        bytes[24..28].copy_from_slice(&10_000_000u32.to_le_bytes());
        bytes[28..32].copy_from_slice(&10_000_000u32.to_le_bytes());
        let crs = parse_dssi_integer_crs(&bytes).unwrap();
        assert_eq!(crs.cmf_x, 10_000_000);
        assert_eq!(crs.cmf_y, 10_000_000);
        assert_eq!(crs.dco_x, 0.0);
        assert_eq!(crs.dco_y, 0.0);
    }

    #[test]
    fn decodes_c2il_when_env_enc_present() {
        let Ok(p) = std::env::var("S101_TEST_ENC") else {
            return;
        };
        let path = std::path::Path::new(&p);
        if !path.exists() {
            return;
        }
        let ds = S101Dataset::load(path).unwrap();
        let (_crs, chains) = extract_c2il_polylines_wgs84(&ds, 50_000);
        assert!(
            !chains.is_empty(),
            "expected at least one C2IL chain in {p}"
        );
    }
}
