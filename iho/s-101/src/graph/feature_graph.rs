use super::Feature;

/// All resolved features in one dataset.
#[derive(Debug)]
pub struct FeatureGraph<'fc> {
    pub features: Vec<Feature<'fc>>,
}
