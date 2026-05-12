//! Catalogue-backed [`PortrayalPipeline`] (stage 1 scaffolding).
//!
//! Holds a loaded S-101 [`PortrayalCatalogue`] plus a selected color palette. On
//! [`reset_for_chart`](CatalogueBackedPortrayal::reset_for_chart) it walks every feature record
//! in the ENC and emits a per-feature [`FeaturePortrayalDraft`] that references the rule entry
//! the catalogue declares — **without** executing any Lua. Driving the rule scripts
//! ("emit a real symbol per feature") is stage 2 and requires an embedded Lua runtime.
//!
//! Until then this scaffolding pins the data model + trait surface so stage 2 plugs in.
//!
//! [`reset_for_chart`]: PortrayalPipeline::reset_for_chart

use s_100::FeatureObjectId;
use s_101::{FeatureGraph, PortrayalCatalogue, RuleAsset, S101Dataset};

use crate::{PortrayError, PortrayalPipeline};

/// Pipeline that resolves features through a loaded [`PortrayalCatalogue`].
#[derive(Debug, Clone)]
pub struct CatalogueBackedPortrayal {
    catalogue: PortrayalCatalogue,
    /// Selected color palette name (e.g. `"Day"`, `"Dusk"`, `"Night"`). Validated against the
    /// catalogue's color profile when set; defaults to `"Day"`.
    palette_name: String,
    /// Display scale denominator; informational for stage 1.
    scale_denominator: Option<u32>,
    /// Drafts produced by the most recent [`reset_for_chart`](PortrayalPipeline::reset_for_chart) or
    /// [`ingest_feature_graph`](CatalogueBackedPortrayal::ingest_feature_graph).
    last_drafts: Vec<FeaturePortrayalDraft>,
}

/// Stage-1 per-feature draft: rule reference without resolved symbology.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeaturePortrayalDraft {
    /// ISO 8211 data-record index when this draft was produced by
    /// [`reset_for_chart`](PortrayalPipeline::reset_for_chart); sequential position
    /// (`0..N-1`) when produced by [`ingest_feature_graph`](CatalogueBackedPortrayal::ingest_feature_graph).
    pub record_index: usize,
    /// `id` of the [`RuleAsset`] this feature dispatches through (currently always
    /// the top-level template — class-specific dispatch needs Lua to evaluate).
    pub rule_id: String,
    /// Present when the draft was built from a [`FeatureGraph`] (`ingest_feature_graph`).
    pub foid: Option<FeatureObjectId>,
    /// Resolved feature-class **alias** from the feature catalogue, or **code** if no alias.
    /// Set only for graph-driven drafts; [`None`] for drafts produced by
    /// [`reset_for_chart`](PortrayalPipeline::reset_for_chart) (raw FRID walk).
    pub feature_class_alias: Option<String>,
}

/// Errors produced when constructing or driving a [`CatalogueBackedPortrayal`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortrayalSetupError {
    /// The portrayal catalogue declares no `TopLevelTemplate` rule (`main`-style entry).
    NoTopLevelRule,
    /// Requested palette is not present in the catalogue's color profile.
    UnknownPalette(String),
}

impl std::fmt::Display for PortrayalSetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoTopLevelRule => write!(f, "portrayal catalogue has no TopLevelTemplate rule"),
            Self::UnknownPalette(p) => write!(f, "unknown color palette: {p}"),
        }
    }
}

impl std::error::Error for PortrayalSetupError {}

impl CatalogueBackedPortrayal {
    /// Construct a pipeline with the catalogue's default palette (`"Day"`).
    pub fn new(catalogue: PortrayalCatalogue) -> Result<Self, PortrayalSetupError> {
        Self::with_palette(catalogue, "Day")
    }

    /// Construct with a specific palette name.
    pub fn with_palette(
        catalogue: PortrayalCatalogue,
        palette_name: impl Into<String>,
    ) -> Result<Self, PortrayalSetupError> {
        if top_level_rule(&catalogue).is_none() {
            return Err(PortrayalSetupError::NoTopLevelRule);
        }
        let palette_name = palette_name.into();
        if catalogue.palette(&palette_name).is_none() {
            // Palette-less catalogues are tolerated; only fail if the catalogue HAS a color
            // profile that lacks the requested palette.
            if catalogue.color_profile.is_some() {
                return Err(PortrayalSetupError::UnknownPalette(palette_name));
            }
        }
        Ok(Self {
            catalogue,
            palette_name,
            scale_denominator: None,
            last_drafts: Vec::new(),
        })
    }

    #[must_use]
    pub fn catalogue(&self) -> &PortrayalCatalogue {
        &self.catalogue
    }

    #[must_use]
    pub fn palette_name(&self) -> &str {
        &self.palette_name
    }

    #[must_use]
    pub fn scale_denominator(&self) -> Option<u32> {
        self.scale_denominator
    }

    /// Drafts produced by the most recent [`reset_for_chart`](Self::reset_for_chart) or
    /// [`ingest_feature_graph`](Self::ingest_feature_graph) call.
    #[must_use]
    pub fn drafts(&self) -> &[FeaturePortrayalDraft] {
        &self.last_drafts
    }

    /// Feature-class candidate rules (`SubTemplate` rows) declared by the catalogue. This is a
    /// best-effort list — the manifest does not distinguish feature-class rules from shared
    /// subroutines (e.g. `Default`, `S100Scripting`); callers cross-reference the feature
    /// catalogue when they need a precise mapping.
    pub fn sub_template_rules(&self) -> impl Iterator<Item = &RuleAsset> {
        self.catalogue.manifest.rules.iter().filter(|r| r.is_sub_template())
    }

    /// Replace the draft list returned by [`drafts`](CatalogueBackedPortrayal::drafts) using
    /// resolved features from `graph` (one draft per feature). Still references the catalogue
    /// **top-level** rule only — same stage-1 contract as
    /// [`reset_for_chart`](PortrayalPipeline::reset_for_chart).
    pub fn ingest_feature_graph(&mut self, graph: &FeatureGraph<'_>) {
        let Some(top) = top_level_rule(&self.catalogue) else {
            self.last_drafts.clear();
            return;
        };
        let rule_id = top.id.clone();
        self.last_drafts = graph
            .features
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let feature_class_alias = f.class.map(|ft| {
                    ft.alias.clone().filter(|a| !a.is_empty()).unwrap_or_else(|| ft.code.clone())
                });
                FeaturePortrayalDraft {
                    record_index: i,
                    rule_id: rule_id.clone(),
                    foid: Some(f.foid),
                    feature_class_alias,
                }
            })
            .collect();
    }
}

impl PortrayalPipeline for CatalogueBackedPortrayal {
    /// Walk every `FRID`-bearing record in `chart` and record a draft referencing the
    /// top-level rule. Stage 2 replaces this with full Lua-driven dispatch.
    fn reset_for_chart(&mut self, chart: &S101Dataset) -> Result<(), PortrayError> {
        let Some(top) = top_level_rule(&self.catalogue) else {
            return Err(PortrayError::UnsupportedScale);
        };
        let rule_id = top.id.clone();
        self.last_drafts = chart
            .iter_raw_feature_records()
            .map(|r| FeaturePortrayalDraft {
                record_index: r.record_index,
                rule_id: rule_id.clone(),
                foid: None,
                feature_class_alias: None,
            })
            .collect();
        Ok(())
    }

    fn set_display_scale(&mut self, scale_denominator: u32) -> Result<(), PortrayError> {
        self.scale_denominator = Some(scale_denominator);
        Ok(())
    }
}

fn top_level_rule(catalogue: &PortrayalCatalogue) -> Option<&RuleAsset> {
    catalogue.manifest.rules.iter().find(|r| r.is_top_level())
}

#[cfg(test)]
mod tests {
    use super::*;
    use s_101::{ColorPalette, ColorProfile, NamedAsset, PortrayalCatalogueManifest, RuleAsset};

    fn fixture_catalogue() -> PortrayalCatalogue {
        PortrayalCatalogue {
            bundle_root: "S-101_Portrayal-Catalogue-1.0.2".to_string(),
            manifest: PortrayalCatalogueManifest {
                product_id: "S-101".to_string(),
                version: "1.0.2".to_string(),
                alert_catalog_file: None,
                color_profile_files: vec!["colorProfile.xml".to_string()],
                symbols: vec![NamedAsset {
                    id: "ACHARE02".to_string(),
                    ..NamedAsset::default()
                }],
                line_styles: vec![],
                area_fills: vec![],
                rules: vec![
                    RuleAsset {
                        id: "main".to_string(),
                        rule_type: Some("TopLevelTemplate".to_string()),
                        description: None,
                    },
                    RuleAsset {
                        id: "AnchorageArea".to_string(),
                        rule_type: Some("SubTemplate".to_string()),
                        description: None,
                    },
                ],
            },
            color_profile: Some(ColorProfile {
                tokens: vec![],
                palettes: vec![ColorPalette {
                    name: "Day".to_string(),
                    css: None,
                    items: vec![],
                }],
            }),
        }
    }

    #[test]
    fn constructs_with_default_palette() {
        let p = CatalogueBackedPortrayal::new(fixture_catalogue()).unwrap();
        assert_eq!(p.palette_name(), "Day");
        assert_eq!(p.sub_template_rules().count(), 1);
    }

    #[test]
    fn rejects_unknown_palette_when_profile_present() {
        let err =
            CatalogueBackedPortrayal::with_palette(fixture_catalogue(), "Twilight").unwrap_err();
        assert_eq!(err, PortrayalSetupError::UnknownPalette("Twilight".into()));
    }

    #[test]
    fn rejects_catalogue_without_top_level_rule() {
        let mut cat = fixture_catalogue();
        cat.manifest.rules.retain(|r| !r.is_top_level());
        let err = CatalogueBackedPortrayal::new(cat).unwrap_err();
        assert_eq!(err, PortrayalSetupError::NoTopLevelRule);
    }

    #[test]
    fn set_display_scale_stores_value() {
        let mut p = CatalogueBackedPortrayal::new(fixture_catalogue()).unwrap();
        p.set_display_scale(50_000).unwrap();
        assert_eq!(p.scale_denominator(), Some(50_000));
    }
}
