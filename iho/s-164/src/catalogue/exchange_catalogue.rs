use super::{CatalogueDiscovery, DatasetDiscovery};

/// Parsed subset of **S100_ExchangeCatalogue** needed for tooling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExchangeCatalogue {
    /// Inner catalogue name (e.g. `DisplayStandard`), not the issuing agency block.
    pub catalogue_identifier: String,
    /// Dataset rows from `datasetDiscoveryMetadata` (ENC cell `.000`, updates, etc.).
    pub datasets: Vec<DatasetDiscovery>,
    /// Catalogue rows from `catalogueDiscoveryMetadata` (feature / portrayal / alert catalogues).
    pub catalogues: Vec<CatalogueDiscovery>,
}
