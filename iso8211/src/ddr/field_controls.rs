use super::{DataStructure, DataType, LexicalLevel};

#[derive(Debug)]
pub struct FieldControls {
    pub(super) data_structure: DataStructure,
    pub(super) data_type: DataType,
    pub(super) escape_sequence: LexicalLevel,
}

impl FieldControls {
    pub fn data_structure(&self) -> DataStructure {
        self.data_structure
    }

    pub fn data_type(&self) -> DataType {
        self.data_type
    }

    pub fn escape_sequence(&self) -> LexicalLevel {
        self.escape_sequence
    }
}
