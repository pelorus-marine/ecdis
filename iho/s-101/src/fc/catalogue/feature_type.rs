#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureType {
    pub code: String,
    pub alias: Option<String>,
    pub permitted_primitives: Vec<String>,
    pub attribute_refs: Vec<String>,
}
