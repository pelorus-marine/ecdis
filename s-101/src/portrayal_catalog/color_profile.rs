//! Top-level `<colorProfile>` document: token declarations + palettes.

use roxmltree::Document;

use super::color_palette::ColorPalette;
use super::color_palette_item::ColorPaletteItem;
use super::color_token_decl::ColorTokenDecl;
use super::error::PortrayalCatalogueError;
use super::xml_util::decode_xml_string;

/// `ColorProfiles/colorProfile.xml` — token declarations plus per-palette items.
#[derive(Debug, Clone, Default)]
pub struct ColorProfile {
    pub tokens: Vec<ColorTokenDecl>,
    pub palettes: Vec<ColorPalette>,
}

impl ColorProfile {
    /// Look up a palette by name (case-insensitive).
    pub fn palette(&self, name: &str) -> Option<&ColorPalette> {
        self.palettes.iter().find(|p| p.name.eq_ignore_ascii_case(name))
    }
}

/// Parse a `colorProfile.xml` body.
pub fn parse_color_profile_xml(xml: &[u8]) -> Result<ColorProfile, PortrayalCatalogueError> {
    let text =
        decode_xml_string(xml).map_err(|_| PortrayalCatalogueError::ColorProfileNotUtf8)?;
    let doc = Document::parse(&text).map_err(PortrayalCatalogueError::ColorProfileXml)?;
    let root = doc
        .descendants()
        .find(|n| n.tag_name().name() == "colorProfile")
        .ok_or(PortrayalCatalogueError::MissingManifestRoot)?;

    let tokens = root
        .descendants()
        .filter(|n| n.tag_name().name() == "color" && n.attribute("token").is_some())
        .map(|c| ColorTokenDecl {
            token: c.attribute("token").unwrap_or("").to_string(),
            name: c.attribute("name").unwrap_or("").to_string(),
            description: c
                .children()
                .find(|n| n.tag_name().name() == "description")
                .and_then(|n| n.text())
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty()),
        })
        .collect();

    let palettes = root
        .descendants()
        .filter(|n| n.tag_name().name() == "palette")
        .map(|p| ColorPalette {
            name: p.attribute("name").unwrap_or("").to_string(),
            css: p.attribute("css").map(str::to_string),
            items: p
                .children()
                .filter(|n| n.tag_name().name() == "item")
                .filter_map(ColorPaletteItem::parse)
                .collect(),
        })
        .collect();

    Ok(ColorProfile { tokens, palettes })
}
