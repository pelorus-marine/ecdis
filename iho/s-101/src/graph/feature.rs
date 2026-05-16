use s_100::{FeatureObjectId, Geometry};

use crate::fc::FeatureType;

use super::ResolvedAttribute;

/// Resolved feature with FC-backed class + attributes and WGS84 geometry.
#[derive(Debug)]
pub struct Feature<'fc> {
    pub foid: FeatureObjectId,
    pub class: Option<&'fc FeatureType>,
    /// Number of **ATTR** tuples carried on the source ISO 8211 feature record.
    pub attr_source_count: usize,
    /// **ATTR** tuples whose `ATIX` does not map to a **simple** attribute in the supplied FC
    /// (e.g. complex attributes, or catalogue edition skew).
    pub skipped_attr_tuples: Vec<(u16, Vec<u8>)>,
    /// Number of **SPAS** fields on the source feature record.
    pub spatial_assoc_count: usize,
    pub attributes: Vec<ResolvedAttribute<'fc>>,
    pub geometry: Geometry,
}
