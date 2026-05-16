//! Feature object identifier (FOID triple) — common across S-100 products.
//!
//! Each public type lives in its own file under `feature_id/`; this `mod.rs` is just
//! the namespace assembly point.

mod feature_object_id;

pub use feature_object_id::FeatureObjectId;
