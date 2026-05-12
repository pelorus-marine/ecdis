//! `<ruleFile>` row from the portrayal-catalogue manifest.

use roxmltree::Node;

use super::xml_util::child_text;

/// One `<ruleFile>` row. `id` is typically a feature-class name (`AnchorageArea`) or a shared
/// subroutine (`Default`, `S100Scripting`, etc.). Distinguishing the two requires inspecting
/// the Lua or cross-referencing the feature catalogue.
#[derive(Debug, Clone, Default)]
pub struct RuleAsset {
    pub id: String,
    /// `TopLevelTemplate` for the dispatch entry point, `SubTemplate` for everything else.
    pub rule_type: Option<String>,
    pub description: Option<String>,
}

impl RuleAsset {
    #[must_use]
    pub fn is_top_level(&self) -> bool {
        matches!(self.rule_type.as_deref(), Some("TopLevelTemplate"))
    }

    #[must_use]
    pub fn is_sub_template(&self) -> bool {
        matches!(self.rule_type.as_deref(), Some("SubTemplate"))
    }

    /// Collect `<ruleFile>` children of the `<rules>` element under `root`.
    pub(crate) fn collect(root: Node<'_, '_>) -> Vec<Self> {
        let Some(rules) = root.children().find(|n| n.tag_name().name() == "rules") else {
            return Vec::new();
        };
        rules
            .children()
            .filter(|n| n.tag_name().name() == "ruleFile")
            .map(|rf| Self {
                id: rf.attribute("id").unwrap_or("").to_string(),
                rule_type: child_text(rf, "ruleType"),
                description: rf
                    .children()
                    .find(|n| n.tag_name().name() == "description")
                    .and_then(|n| n.text())
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty()),
            })
            .collect()
    }
}
