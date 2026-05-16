/// One **S100_CatalogueDiscoveryMetadata** block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogueDiscovery {
    pub file_uri: String,
    pub product_identifier: Option<String>,
    /// Catalogue scope as advertised by the issuer (`featureCatalogue`, `portrayalCatalogue`, …).
    /// Note: IHO S-164 v1.2.0 labels portrayal catalogues as `featureCatalogue`; combine with
    /// [`compressed`](Self::compressed) and filename to distinguish.
    pub scope: Option<String>,
    /// Whether the file is a zip bundle (Part 9 portrayal catalogues are typically `true`).
    pub compressed: Option<bool>,
}
