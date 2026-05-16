use super::Classification;

/// One exchange set discovered in the corpus zip.
#[derive(Debug, Clone)]
pub struct ExchangeSetEntry {
    /// Zip-prefix ending at the parent of `S100_ROOT/` (always ends with `/`, unless empty).
    pub prefix: String,
    /// Inner catalogue identifier from `<identifier>` (e.g. `DisplayStandard`).
    pub catalogue_identifier: String,
    /// Scenario classification derived from [`prefix`](Self::prefix).
    pub classification: Classification,
}
