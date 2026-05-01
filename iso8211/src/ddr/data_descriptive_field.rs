use std::{
    io::{Read, Seek},
    str::FromStr,
};

use super::{DataStructure, DataType, FieldControls, FormatControls, LexicalLevel};
use crate::{DirectoryEntry, Iso8211Error, Reader, Result};

#[derive(Debug)]
pub struct DataDescriptiveField {
    field_controls: FieldControls,
    field_name: String,
    array_descriptor: String,
    format_controls: FormatControls,
}

impl DataDescriptiveField {
    pub fn read<T: Read + Seek>(
        reader: &mut Reader<T>,
        //FIXME: Find out how directory entry should be used
        _entry: &DirectoryEntry,
    ) -> Result<DataDescriptiveField> {
        // Data structure code (b11,b14,2b12,b11,{3b12,b11,A})
        let data_structure = reader.read_char()?;
        let data_structure = DataStructure::from_str(data_structure.to_string().as_str())?;

        // Data type code
        let data_type = reader.read_str(1)?;
        let data_type = DataType::from_str(data_type.as_str())?;

        // Auxiliary controls must be "00"
        let auxiliary_controls = reader.read_str(2)?;
        if auxiliary_controls != "00" {
            return Err(Iso8211Error::Parse(format!(
                "Invalid Auxiliary Controls: {}",
                auxiliary_controls
            )));
        }

        // Printable graphics must be ";&"
        let printable_graphics = reader.read_str(2)?;
        if printable_graphics != ";&" {
            return Err(Iso8211Error::Parse(format!(
                "Invalid Printable Graphics: {}",
                printable_graphics
            )));
        }

        // Truncated escape sequence
        let escape_sequence = reader.read_str(3)?;
        let escape_sequence = LexicalLevel::from_str(escape_sequence.as_str())?;

        let field_name = reader.read_str_ut()?;

        let array_descriptor = reader.read_str_ut()?;

        let format_controls = reader.read_str_ft()?;
        let format_controls = FormatControls::from_str(format_controls.as_str())?;

        let field_controls = FieldControls {
            data_structure,
            data_type,
            escape_sequence,
        };

        Ok(DataDescriptiveField {
            field_controls,
            field_name,
            array_descriptor,
            format_controls,
        })
    }

    pub fn field_controls(&self) -> &FieldControls {
        &self.field_controls
    }

    pub fn field_name(&self) -> &str {
        &self.field_name
    }

    pub fn array_descriptor(&self) -> &str {
        &self.array_descriptor
    }

    pub fn format_controls(&self) -> &FormatControls {
        &self.format_controls
    }
}
