use super::ListedValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleAttribute {
    pub code: String,
    pub alias: Option<String>,
    pub value_type: String,
    pub source_identifier: Option<u32>,
    pub listed_values: Vec<ListedValue>,
}
