use super::Classification;

/// One catalogue row (`S100_CatalogueDiscoveryMetadata`), e.g. feature / portrayal catalogue.
#[derive(Debug, Clone)]
pub struct CatalogueEntry {
    /// Index into [`super::Corpus::exchange_sets`].
    pub exchange_set_index: usize,
    pub product_identifier: Option<String>,
    /// `fileName` URI from the catalogue (`file:/…`).
    pub file_uri: String,
    /// Fully resolved entry path inside the zip (input to [`super::Corpus::read_catalogue`]).
    pub zip_path: String,
    /// Catalogue scope (`featureCatalogue`, `portrayalCatalogue`, …) as advertised.
    pub scope: Option<String>,
    /// Whether the catalogue file is a zip bundle (S-100 Part 9 portrayal catalogues are).
    pub compressed: Option<bool>,
    pub classification: Classification,
}

impl CatalogueEntry {
    /// True for `fileName`s containing `Portrayal` (heuristic — IHO labels both FC and PC as
    /// `scope="featureCatalogue"` in S-164 v1.2.0, so distinguish by filename + [`compressed`](Self::compressed)).
    #[must_use]
    pub fn looks_like_portrayal_catalogue(&self) -> bool {
        self.file_uri
            .rsplit('/')
            .next()
            .unwrap_or(&self.file_uri)
            .to_ascii_lowercase()
            .contains("portrayal")
    }
}
