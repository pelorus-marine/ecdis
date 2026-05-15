//! Pinned **IHO S-101 product specification** edition targeted by this crate build.
//!
//! Full **feature catalogue** (FC) XML binding is not implemented yet; callers should load FC
//! separately and ensure it matches the ENC producer’s catalogue edition before trusting semantic
//! decode results.

/// Edition label for **S-101 ENC product specification** this codebase is aligned with (informative).
///
/// Update when intentionally migrating to a newer registered IHO edition.
pub const TARGET_PRODUCT_SPECIFICATION_EDITION: &str = "1.2.0";

/// Placeholder until FC XML is parsed: human-readable expectation that FC binding must match ENC.
pub const FEATURE_CATALOGUE_BINDING_NOTE: &str =
    "Bind FC XML edition to dataset DSID fields before FC-driven attribute interpretation.";
