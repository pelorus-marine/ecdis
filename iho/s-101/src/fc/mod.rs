//! **S-101 Feature Catalogue** XML (subset for decoding + graph resolution).

mod catalogue;
mod edition;

pub use catalogue::{
    ComplexAttribute, FeatureCatalogue, FeatureType, InformationType, ListedValue, SimpleAttribute,
};
pub use edition::{FcEditionSummary, parse_fc_edition_summary};

/// Errors from FC XML parsing / edition probe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FcCatalogParseError {
    Utf8(std::str::Utf8Error),
    MissingProductId,
    MissingVersionNumber,
    MissingVersionDate,
    Xml(String),
}

impl std::fmt::Display for FcCatalogParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Utf8(e) => write!(f, "invalid UTF-8 in FC XML: {e}"),
            Self::MissingProductId => write!(f, "missing S100FC:productId element"),
            Self::MissingVersionNumber => write!(f, "missing S100FC:versionNumber element"),
            Self::MissingVersionDate => write!(f, "missing S100FC:versionDate element"),
            Self::Xml(s) => write!(f, "XML parse error: {s}"),
        }
    }
}

impl std::error::Error for FcCatalogParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Utf8(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::str::Utf8Error> for FcCatalogParseError {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::Utf8(e)
    }
}

impl From<roxmltree::Error> for FcCatalogParseError {
    fn from(e: roxmltree::Error) -> Self {
        Self::Xml(e.to_string())
    }
}
