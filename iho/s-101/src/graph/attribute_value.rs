/// Attribute value resolved through the feature catalogue.
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {
    Boolean(bool),
    Integer(i64),
    Real(f64),
    Text(String),
    Enumeration { code: u32, label: Option<String> },
    Raw(Vec<u8>),
}
