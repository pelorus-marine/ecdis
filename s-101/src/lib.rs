//! IHO **S-101** Electronic Navigational Chart (ENC) — first decode slice.
//!
//! Loads **ISO 8211** exchange data via [`iso8211`], validates that the DDR describes an
//! S-101-style **DSID** field, and exposes typed records, feature-catalogue XML parsing, and a
//! **feature graph** (FC-backed attributes + WGS84 geometry).
//!
//! # Example
//!
//! ```ignore
//! use s_101::{FeatureCatalogue, S101Dataset};
//!
//! let enc = S101Dataset::load("path/to/dataset.000")?;
//! let fc_bytes = std::fs::read("path/to/S-101_FC.xml")?;
//! let fc = FeatureCatalogue::parse_xml(&fc_bytes)?;
//! let graph = enc.build_feature_graph(&fc)?;
//! println!("features: {}", graph.features.len());
//! ```

#![forbid(unsafe_code)]

mod binary;
mod dataset;
mod decode;
pub mod edition;
mod error;
pub mod fc;
pub mod geometry;
pub mod graph;
pub mod portrayal_catalog;
pub mod record;
pub mod semantic;

pub use dataset::S101Dataset;
pub use decode::{field_payload, record_field_payload};
pub use edition::{FEATURE_CATALOGUE_BINDING_NOTE, TARGET_PRODUCT_SPECIFICATION_EDITION};
pub use error::S101Error;
pub use fc::{
    ComplexAttribute, FcCatalogParseError, FcEditionSummary, FeatureCatalogue, FeatureType,
    InformationType, ListedValue, SimpleAttribute, parse_fc_edition_summary,
};
pub use geometry::{IntegerCrsParameters, extract_c2il_polylines_wgs84};
pub use graph::{AttributeValue, Feature, FeatureGraph, ResolvedAttribute};
pub use portrayal_catalog::{
    ColorPalette, ColorPaletteItem, ColorProfile, ColorTokenDecl, NamedAsset, PortrayalCatalogue,
    PortrayalCatalogueError, PortrayalCatalogueManifest, RuleAsset,
};
pub use s_100::{Curve2D, FeatureObjectId, Geometry, MultiPoint2D, Point2D, Surface2D};
pub use semantic::{FeatureCataloguePin, FeatureInventorySummary, RawFeatureRecordRef};
