//! **CSID** coordinate reference record.

use crate::record::identifier::RecordIdentifier;

#[derive(Debug, Clone, PartialEq)]
pub struct CoordinateSystemRecord {
    pub record_index: usize,
    pub id: RecordIdentifier,
    pub crsh: Vec<Vec<u8>>,
    pub csax: Vec<Vec<u8>>,
    pub vdat: Vec<Vec<u8>>,
    pub tail: Vec<(String, Vec<u8>)>,
}

impl CoordinateSystemRecord {
    #[must_use]
    pub fn parse(record_index: usize, fields: &[(String, Vec<u8>)]) -> Option<Self> {
        let mut csid_payload = None;
        let mut crsh = Vec::new();
        let mut csax = Vec::new();
        let mut vdat = Vec::new();
        let mut tail = Vec::new();
        for (tag, data) in fields {
            match tag.as_str() {
                "CSID" => csid_payload = Some(data.as_slice()),
                "CRSH" => crsh.push(data.clone()),
                "CSAX" => csax.push(data.clone()),
                "VDAT" => vdat.push(data.clone()),
                _ => tail.push((tag.clone(), data.clone())),
            }
        }
        let id = RecordIdentifier::parse(csid_payload?)?;
        Some(Self {
            record_index,
            id,
            crsh,
            csax,
            vdat,
            tail,
        })
    }
}
