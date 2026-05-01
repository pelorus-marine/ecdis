use std::io::{Read, Seek};

use super::DataField;
use crate::{Directory, Leader, Reader, Result};

pub struct DataRecord {
    /// ISO 8211 field tags in directory order (same length as [`Self::data_fields`]).
    pub field_tags: Vec<String>,
    pub data_fields: Vec<DataField>,
}

impl DataRecord {
    pub fn read<T: Read + Seek>(reader: &mut Reader<T>) -> Result<DataRecord> {
        let leader = Leader::read_dr(reader)?;

        let directory = Directory::read(reader, &leader)?;

        let entries = directory.entries();
        let mut field_tags = Vec::with_capacity(entries.len());
        let mut data_fields = Vec::with_capacity(entries.len());
        for entry in entries {
            field_tags.push(entry.field_tag().to_string());
            let df = DataField::read(reader, entry)?;
            data_fields.push(df);
        }

        Ok(DataRecord {
            field_tags,
            data_fields,
        })
    }

    pub fn data_fields(&self) -> &[DataField] {
        &self.data_fields
    }
}
