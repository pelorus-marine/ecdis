//! Feature **geometry graph**: resolve `FRID` + `SPAS` against typed spatial records and FC.
//!
//! **Curve** lookups accept **RIAS** / **CUCO** references with `rrn == 125` when the matching
//! **CRID** record uses `rcnm == 120` (IHO **S-64 v1.2.0** quirk). **MRID** + **C3IL** rows are
//! indexed for multipoint `SPAS` references (`rrn == 115`).
//!
//! Each public type lives in its own file under `graph/`; this `mod.rs` is just
//! the namespace assembly point.

mod attribute_value;
mod build;
mod feature;
mod feature_graph;
mod resolved_attribute;

pub use attribute_value::AttributeValue;
pub use feature::Feature;
pub use feature_graph::FeatureGraph;
pub use resolved_attribute::ResolvedAttribute;

use crate::S101Error;
use crate::dataset::S101Dataset;
use crate::fc::FeatureCatalogue;

impl S101Dataset {
    /// Build a [`FeatureGraph`] using catalogue-driven attribute typing.
    pub fn build_feature_graph<'fc>(
        &self,
        fc: &'fc FeatureCatalogue,
    ) -> Result<FeatureGraph<'fc>, S101Error> {
        build::build_feature_graph_inner(self, fc)
    }
}
