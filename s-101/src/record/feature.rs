//! **FRID** feature records (`FOID`, `ATTR`, `SPAS`, …).

use crate::decode::record_field_payload;
use crate::record::attribute::parse_attr_tuples;
use crate::record::foid::parse_foid;
use crate::record::frid::FridHeader;
use crate::record::spatial_ref::SpasRef;
use iso8211::dr::DataRecord;

#[derive(Debug, Clone, PartialEq)]
pub struct FeatureRecord {
    pub record_index: usize,
    pub frid: FridHeader,
    pub foid: s_100::FeatureObjectId,
    pub attributes: Vec<crate::record::attribute::RawAttributeTuple>,
    pub spatial: Vec<SpasRef>,
    pub tail: Vec<(String, Vec<u8>)>,
}

impl FeatureRecord {
    #[must_use]
    #[allow(clippy::collapsible_if)]
    pub fn parse(record_index: usize, rec: &DataRecord) -> Option<Self> {
        let frid = FridHeader::parse(record_field_payload(rec, "FRID")?)?;
        let foid = parse_foid(record_field_payload(rec, "FOID")?)?;
        let mut attributes = Vec::new();
        for (t, df) in rec.field_tags.iter().zip(rec.data_fields.iter()) {
            if t == "ATTR" {
                attributes.extend(parse_attr_tuples(df.user_data()));
            }
        }
        let mut spatial = Vec::new();
        for (t, df) in rec.field_tags.iter().zip(rec.data_fields.iter()) {
            if t == "SPAS" {
                if let Some(s) = SpasRef::parse(df.user_data()) {
                    spatial.push(s);
                }
            }
        }
        let mut tail = Vec::new();
        for (t, df) in rec.field_tags.iter().zip(rec.data_fields.iter()) {
            if matches!(t.as_str(), "INAS" | "FASC") {
                tail.push((t.clone(), df.user_data().to_vec()));
            }
        }
        Some(Self {
            record_index,
            frid,
            foid,
            attributes,
            spatial,
            tail,
        })
    }
}
