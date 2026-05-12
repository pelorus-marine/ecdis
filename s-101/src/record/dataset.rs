//! **DSID** and related discovery fields on record 0.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetDescriptionRecord {
    pub record_index: usize,
    pub dsid: Vec<u8>,
    pub dssi: Option<Vec<u8>>,
    pub ftcs: Option<Vec<u8>>,
    pub tail: Vec<(String, Vec<u8>)>,
}

impl DatasetDescriptionRecord {
    #[must_use]
    pub fn parse(record_index: usize, fields: &[(String, Vec<u8>)]) -> Option<Self> {
        let mut dsid = None;
        let mut dssi = None;
        let mut ftcs = None;
        let mut tail = Vec::new();
        for (tag, data) in fields {
            match tag.as_str() {
                "DSID" => dsid = Some(data.clone()),
                "DSSI" => dssi = Some(data.clone()),
                "FTCS" => ftcs = Some(data.clone()),
                _ => tail.push((tag.clone(), data.clone())),
            }
        }
        Some(Self {
            record_index,
            dsid: dsid?,
            dssi,
            ftcs,
            tail,
        })
    }
}
