//! Generic asset row (`<symbol>`, `<lineStyle>`, `<areaFill>`).

use roxmltree::Node;

use super::xml_util::child_text;

/// Catalogued asset with an `id`, optional file name, and asset format string.
#[derive(Debug, Clone, Default)]
pub struct NamedAsset {
    pub id: String,
    pub file_name: Option<String>,
    pub file_format: Option<String>,
    pub description: Option<String>,
}

impl NamedAsset {
    /// Collect every direct child of `root` named `container_local`, then each `item_local`
    /// child as a [`NamedAsset`].
    pub(crate) fn collect(
        root: Node<'_, '_>,
        container_local: &str,
        item_local: &str,
    ) -> Vec<Self> {
        let Some(container) = root.children().find(|n| n.tag_name().name() == container_local)
        else {
            return Vec::new();
        };
        container
            .children()
            .filter(|n| n.tag_name().name() == item_local)
            .map(|item| Self {
                id: item.attribute("id").unwrap_or("").to_string(),
                file_name: child_text(item, "fileName"),
                file_format: child_text(item, "fileFormat"),
                description: item
                    .children()
                    .find(|n| n.tag_name().name() == "description")
                    .and_then(|d| child_text(d, "name"))
                    .or_else(|| {
                        item.children()
                            .find(|n| n.tag_name().name() == "description")
                            .and_then(|d| child_text(d, "description"))
                    }),
            })
            .collect()
    }
}
