use std::io::{Read, Seek};

use crate::{Reader, Result};

#[derive(Debug, Copy, Clone)]
pub struct EntryMap {
    /// Size Of Field Length Field
    field_length: u8,
    /// Size Of Field Position Field
    field_position: u8,
    /// Size Of Field Tag Field
    field_tag: u8,
}

impl EntryMap {
    pub fn read<T: Read + Seek>(reader: &mut Reader<T>) -> Result<EntryMap> {
        let field_length = reader.read_u8_str(1)?;

        let field_position = reader.read_u8_str(1)?;

        reader.read_char()?;

        let field_tag = reader.read_u8_str(1)?;

        Ok(EntryMap {
            field_length,
            field_position,
            field_tag,
        })
    }

    /// Size Of Field Length Field
    pub fn field_length(&self) -> u8 {
        self.field_length
    }

    /// Size Of Field Position Field
    pub fn field_position(&self) -> u8 {
        self.field_position
    }

    /// Size Of Field Tag Field
    pub fn field_tag(&self) -> u8 {
        self.field_tag
    }
}
