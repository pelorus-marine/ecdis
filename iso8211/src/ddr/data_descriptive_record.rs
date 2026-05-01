use std::io::{Read, Seek};

use super::{DataDescriptiveField, FileControlField};
use crate::{Directory, Iso8211Error, Leader, Reader, Result};

#[derive(Debug)]
pub struct DataDescriptiveRecord {
    file_control_field: FileControlField,
    data_descriptive_fields: Vec<DataDescriptiveField>,
}

impl DataDescriptiveRecord {
    pub fn read<T: Read + Seek>(reader: &mut Reader<T>) -> Result<DataDescriptiveRecord> {
        let leader = Leader::read_ddr(reader)?;

        let directory = Directory::read(reader, &leader)?;

        let entries = directory.entries();
        let Some(first) = entries.first() else {
            return Err(Iso8211Error::Parse(
                "DDR directory is empty (expected at least file control field entry)".into(),
            ));
        };

        let file_control_field = FileControlField::read(reader, &leader, first)?;

        let mut data_descriptive_fields: Vec<DataDescriptiveField> =
            Vec::with_capacity(entries.len() - 1);
        for i in entries.iter().skip(1) {
            let ddf = DataDescriptiveField::read(reader, i)?;
            data_descriptive_fields.push(ddf);
        }

        Ok(DataDescriptiveRecord {
            file_control_field,
            data_descriptive_fields,
        })
    }

    pub fn file_control_field(&self) -> &FileControlField {
        &self.file_control_field
    }

    pub fn data_descriptive_fields(&self) -> &[DataDescriptiveField] {
        &self.data_descriptive_fields
    }
}
