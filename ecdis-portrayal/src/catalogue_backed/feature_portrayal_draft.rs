use s_100::FeatureObjectId;

/// Stage-1 per-feature draft: rule reference without resolved symbology.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeaturePortrayalDraft {
    /// ISO 8211 data-record index when this draft was produced by
    /// [`reset_for_chart`](crate::portrayal::PortrayalPipeline::reset_for_chart); sequential position
    /// (`0..N-1`) when produced by [`ingest_feature_graph`](super::CatalogueBackedPortrayal::ingest_feature_graph).
    pub record_index: usize,
    /// `id` of the [`RuleAsset`](s_101::RuleAsset) this feature dispatches through (currently always
    /// the top-level template — class-specific dispatch needs Lua to evaluate).
    pub rule_id: String,
    /// Present when the draft was built from a [`FeatureGraph`](s_101::FeatureGraph) (`ingest_feature_graph`).
    pub foid: Option<FeatureObjectId>,
    /// Resolved feature-class **alias** from the feature catalogue, or **code** if no alias.
    /// Set only for graph-driven drafts; [`None`] for drafts produced by
    /// [`reset_for_chart`](crate::portrayal::PortrayalPipeline::reset_for_chart) (raw FRID walk).
    pub feature_class_alias: Option<String>,
}
