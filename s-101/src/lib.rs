//! IHO **S-101** Electronic Navigational Chart (ENC) — first decode slice.
//!
//! Loads **ISO 8211** exchange data via [`iso8211`], validates that the DDR describes an
//! S-101-style **DSID** field, and exposes tagged record payloads for higher-level feature
//! decoding (future work).
//!
//! # Example
//!
//! ```no_run
//! use s_101::S101Dataset;
//!
//! let enc = S101Dataset::load("path/to/dataset.000")?;
//! println!("records: {}", enc.record_count());
//! # Ok::<(), s_101::S101Error>(())
//! ```

#![forbid(unsafe_code)]

mod dataset;
mod decode;
pub mod edition;
mod error;
pub mod fc_catalog;
pub mod geometry;
pub mod semantic;

pub use dataset::S101Dataset;
pub use decode::{field_payload, record_field_payload};
pub use edition::{FEATURE_CATALOGUE_BINDING_NOTE, TARGET_PRODUCT_SPECIFICATION_EDITION};
pub use error::S101Error;
pub use fc_catalog::{parse_fc_edition_summary, FcCatalogParseError, FcEditionSummary};
pub use geometry::{
    decode_c2il_integer_pairs, extract_c2il_polylines_wgs84, parse_dssi_integer_crs,
    trim_iso8211_unit_term, IntegerCrsParameters,
};
pub use semantic::{FeatureCataloguePin, FeatureInventorySummary, RawFeatureRecordRef};
