use std::io::{Read, Seek};

use crate::{DirectoryEntry, Reader, Result};

pub struct DataField {
    user_data: Vec<u8>,
}

impl DataField {
    pub fn read<T: Read + Seek>(
        reader: &mut Reader<T>,
        entry: &DirectoryEntry,
    ) -> Result<DataField> {
        let user_data = reader.read_bytes(entry.field_length() as usize)?;

        Ok(DataField { user_data })
    }

    /// Build a field from already-extracted payload bytes (tests and higher-level decoders).
    pub fn from_vec(user_data: Vec<u8>) -> Self {
        Self { user_data }
    }

    pub fn user_data(&self) -> &[u8] {
        &self.user_data
    }
}
