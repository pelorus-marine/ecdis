//! **S-101 Feature Catalogue** XML (subset for decoding + graph resolution).
//!
//! Each public type lives in its own file under `fc/`; this `mod.rs` is just
//! the namespace assembly point.

mod catalogue;
mod edition;
mod error;

pub use catalogue::{
    ComplexAttribute, FeatureCatalogue, FeatureType, InformationType, ListedValue, SimpleAttribute,
};
pub use edition::{FcEditionSummary, parse_fc_edition_summary};
pub use error::FcCatalogParseError;
