//! Chart chrome colours resolved from a portrayal palette or built-in fallbacks.

use s_101::ColorPalette;

use crate::display_mode::DisplayMode;

/// sRGB triplet for UI / vector layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// `#rrggbb` for Slint / SVG.
    #[must_use]
    pub fn to_hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

/// Colours for chart background, linework, and HUD accents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChartTheme {
    pub background: Rgb,
    pub chart_stroke: Rgb,
    pub surface_fill: Rgb,
    pub surface_stroke: Rgb,
    pub point_fill: Rgb,
    pub own_ship: Rgb,
    pub heading: Rgb,
    pub hud_primary: Rgb,
    pub hud_muted: Rgb,
    pub border: Rgb,
    /// Stylesheet file from the active palette, if any.
    pub palette_css: Option<String>,
    pub from_catalogue: bool,
}

impl ChartTheme {
    /// Resolve theme for `mode`, optionally using a catalogue palette.
    #[must_use]
    pub fn resolve(mode: DisplayMode, palette: Option<&ColorPalette>) -> Self {
        let fallback = fallback_theme(mode);
        let Some(palette) = palette else {
            return fallback;
        };

        let pick = |token: &str, default: Rgb| -> Rgb {
            palette.srgb(token).map(|(r, g, b)| Rgb::new(r, g, b)).unwrap_or(default)
        };

        Self {
            background: pick("DEPDW", fallback.background),
            chart_stroke: pick("CSTLN", fallback.chart_stroke),
            surface_fill: pick("DEPVS", fallback.surface_fill),
            surface_stroke: pick("DEPCN", fallback.surface_stroke),
            point_fill: pick("CHYLW", fallback.point_fill),
            own_ship: pick("CHYLW", fallback.own_ship),
            heading: pick("CHMGD", fallback.heading),
            hud_primary: pick("CHBLK", fallback.hud_primary),
            hud_muted: pick("CHGRD", fallback.hud_muted),
            border: pick("CHGRD", fallback.border),
            palette_css: palette.css.clone(),
            from_catalogue: true,
        }
    }
}

fn fallback_theme(mode: DisplayMode) -> ChartTheme {
    match mode {
        DisplayMode::Day => ChartTheme {
            background: Rgb::new(201, 237, 255),
            chart_stroke: Rgb::new(76, 91, 99),
            surface_fill: Rgb::new(88, 175, 156),
            surface_stroke: Rgb::new(46, 125, 82),
            point_fill: Rgb::new(255, 170, 68),
            own_ship: Rgb::new(255, 170, 68),
            heading: Rgb::new(136, 221, 255),
            hud_primary: Rgb::new(0, 0, 0),
            hud_muted: Rgb::new(118, 140, 151),
            border: Rgb::new(68, 68, 68),
            palette_css: None,
            from_catalogue: false,
        },
        DisplayMode::Dusk => ChartTheme {
            background: Rgb::new(30, 65, 101),
            chart_stroke: Rgb::new(107, 127, 137),
            surface_fill: Rgb::new(35, 76, 68),
            surface_stroke: Rgb::new(33, 130, 92),
            point_fill: Rgb::new(139, 139, 31),
            own_ship: Rgb::new(227, 128, 57),
            heading: Rgb::new(165, 165, 39),
            hud_primary: Rgb::new(201, 237, 255),
            hud_muted: Rgb::new(140, 166, 179),
            border: Rgb::new(76, 91, 99),
            palette_css: None,
            from_catalogue: false,
        },
        DisplayMode::Night => ChartTheme {
            background: Rgb::new(14, 19, 21),
            chart_stroke: Rgb::new(108, 160, 212),
            surface_fill: Rgb::new(35, 76, 68),
            surface_stroke: Rgb::new(47, 142, 32),
            point_fill: Rgb::new(255, 170, 68),
            own_ship: Rgb::new(255, 170, 68),
            heading: Rgb::new(136, 221, 255),
            hud_primary: Rgb::new(170, 187, 204),
            hud_muted: Rgb::new(118, 140, 151),
            border: Rgb::new(68, 68, 68),
            palette_css: None,
            from_catalogue: false,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use s_101::{ColorPalette, ColorPaletteItem};

    #[test]
    fn catalogue_overrides_token() {
        let palette = ColorPalette {
            name: "Night".to_string(),
            css: Some("nightSvgStyle.css".to_string()),
            items: vec![ColorPaletteItem {
                token: "CHRED".to_string(),
                srgb: (96, 0, 0),
                cie_xy_l: None,
            }],
        };
        let theme = ChartTheme::resolve(DisplayMode::Night, Some(&palette));
        assert!(theme.from_catalogue);
        assert_eq!(theme.palette_css.as_deref(), Some("nightSvgStyle.css"));
    }
}
