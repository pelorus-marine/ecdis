/// Feature catalogue edition pin used **together with** [`crate::edition`] constants.
///
/// FC XML ingestion lives outside this first slice; applications should set these from DSID + FC file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureCataloguePin {
    pub product_specification_edition: String,
    pub feature_catalogue_edition: String,
}

impl Default for FeatureCataloguePin {
    fn default() -> Self {
        Self {
            product_specification_edition: crate::edition::TARGET_PRODUCT_SPECIFICATION_EDITION
                .to_string(),
            feature_catalogue_edition: String::new(),
        }
    }
}
