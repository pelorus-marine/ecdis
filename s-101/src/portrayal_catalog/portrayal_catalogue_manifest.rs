//! Parsed `portrayal_catalogue.xml` summary (no rule / symbol content).

use roxmltree::Document;

use super::error::PortrayalCatalogueError;
use super::named_asset::NamedAsset;
use super::rule_asset::RuleAsset;
use super::xml_util::decode_xml_string;

/// Parsed `portrayal_catalogue.xml` summary (no rule / symbol content).
#[derive(Debug, Clone, Default)]
pub struct PortrayalCatalogueManifest {
    /// `productId` attribute (e.g. `S-101`).
    pub product_id: String,
    /// `version` attribute (e.g. `1.0.2`).
    pub version: String,
    /// `<alertCatalog><fileName>…</fileName></alertCatalog>` body when present.
    pub alert_catalog_file: Option<String>,
    /// File names referenced by `<colorProfiles><colorProfile>…</colorProfile></colorProfiles>`.
    pub color_profile_files: Vec<String>,
    pub symbols: Vec<NamedAsset>,
    pub line_styles: Vec<NamedAsset>,
    pub area_fills: Vec<NamedAsset>,
    pub rules: Vec<RuleAsset>,
}

/// Parse the `portrayal_catalogue.xml` body.
pub fn parse_manifest_xml(
    xml: &[u8],
) -> Result<PortrayalCatalogueManifest, PortrayalCatalogueError> {
    let text = decode_xml_string(xml).map_err(|_| PortrayalCatalogueError::ManifestNotUtf8)?;
    let doc = Document::parse(&text)?;
    let root = doc
        .descendants()
        .find(|n| n.tag_name().name() == "portrayalCatalog")
        .ok_or(PortrayalCatalogueError::MissingManifestRoot)?;

    let product_id = root.attribute("productId").unwrap_or("").to_string();
    let version = root.attribute("version").unwrap_or("").to_string();

    let alert_catalog_file = root
        .children()
        .find(|n| n.tag_name().name() == "alertCatalog")
        .and_then(|n| n.children().find(|c| c.tag_name().name() == "fileName"))
        .and_then(|n| n.text())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let color_profile_files = root
        .descendants()
        .filter(|n| n.tag_name().name() == "colorProfile")
        .filter_map(|cp| {
            cp.children()
                .find(|c| c.tag_name().name() == "fileName")
                .and_then(|n| n.text())
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
        })
        .collect();

    let symbols = NamedAsset::collect(root, "symbols", "symbol");
    let line_styles = NamedAsset::collect(root, "lineStyles", "lineStyle");
    let area_fills = NamedAsset::collect(root, "areaFills", "areaFill");
    let rules = RuleAsset::collect(root);

    Ok(PortrayalCatalogueManifest {
        product_id,
        version,
        alert_catalog_file,
        color_profile_files,
        symbols,
        line_styles,
        area_fills,
        rules,
    })
}
