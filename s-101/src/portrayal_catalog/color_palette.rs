//! `<palette name="…" css="…">` resolution table.

use super::color_palette_item::ColorPaletteItem;

/// `<palette name="Day" css="daySvgStyle.css">…</palette>` resolution table.
#[derive(Debug, Clone, Default)]
pub struct ColorPalette {
    pub name: String,
    pub css: Option<String>,
    pub items: Vec<ColorPaletteItem>,
}

impl ColorPalette {
    /// Look up the sRGB triple for `token`, if present.
    #[must_use]
    pub fn srgb(&self, token: &str) -> Option<(u8, u8, u8)> {
        self.items.iter().find(|i| i.token == token).map(|i| i.srgb)
    }
}
