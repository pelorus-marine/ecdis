//! One row inside a `<palette>` — token → sRGB (+ optional CIE chromaticity).

use roxmltree::Node;

use super::xml_util::child_text;

/// Per-palette resolution row.
#[derive(Debug, Clone)]
pub struct ColorPaletteItem {
    pub token: String,
    pub srgb: (u8, u8, u8),
    /// CIE 1931 chromaticity (`x`, `y`, `L`) when the profile declares it.
    pub cie_xy_l: Option<(f64, f64, f64)>,
}

impl ColorPaletteItem {
    pub(crate) fn parse(item: Node<'_, '_>) -> Option<Self> {
        let token = item.attribute("token")?.to_string();
        let srgb = item.children().find(|n| n.tag_name().name() == "srgb")?;
        let r: u8 = child_text(srgb, "red")?.parse().ok()?;
        let g: u8 = child_text(srgb, "green")?.parse().ok()?;
        let b: u8 = child_text(srgb, "blue")?.parse().ok()?;
        let cie_xy_l = item
            .children()
            .find(|n| n.tag_name().name() == "cie")
            .and_then(|cie| cie.children().find(|n| n.tag_name().name() == "xyL"))
            .and_then(|xy_l| {
                let x: f64 = child_text(xy_l, "x")?.parse().ok()?;
                let y: f64 = child_text(xy_l, "y")?.parse().ok()?;
                let l: f64 = child_text(xy_l, "L")?.parse().ok()?;
                Some((x, y, l))
            });
        Some(Self {
            token,
            srgb: (r, g, b),
            cie_xy_l,
        })
    }
}
