use super::Classification;

/// One dataset row from an exchange catalogue, with its resolved zip path.
#[derive(Debug, Clone)]
pub struct DatasetEntry {
    /// Index into [`super::Corpus::exchange_sets`].
    pub exchange_set_index: usize,
    /// `productIdentifier` from the catalogue (`Some("S-101")`, `Some("S-102")`, etc.).
    pub product_identifier: Option<String>,
    /// `fileName` URI from the catalogue (`file:/…`).
    pub file_uri: String,
    /// Fully resolved entry path inside the zip (input to [`super::Corpus::read_dataset`]).
    pub zip_path: String,
    /// Classification copied from the owning exchange set for convenience.
    pub classification: Classification,
}
