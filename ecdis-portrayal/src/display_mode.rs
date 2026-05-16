//! Mariner display mode (Day / Dusk / Night) mapped to portrayal catalogue palette names.

/// ECDIS display mode — maps to `<palette name="…">` in `colorProfile.xml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DisplayMode {
    #[default]
    Day,
    Dusk,
    Night,
}

impl DisplayMode {
    pub const ALL: [Self; 3] = [Self::Day, Self::Dusk, Self::Night];

    /// Catalogue palette name for this mode.
    #[must_use]
    pub fn palette_name(self) -> &'static str {
        match self {
            Self::Day => "Day",
            Self::Dusk => "Dusk",
            Self::Night => "Night",
        }
    }

    /// Parse CLI / env values (`day`, `DUSK`, …).
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "day" => Some(Self::Day),
            "dusk" => Some(Self::Dusk),
            "night" => Some(Self::Night),
            _ => None,
        }
    }

    /// Cycle Day → Dusk → Night → Day.
    #[must_use]
    pub fn cycle(self) -> Self {
        match self {
            Self::Day => Self::Dusk,
            Self::Dusk => Self::Night,
            Self::Night => Self::Day,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_cycle() {
        assert_eq!(DisplayMode::parse("NIGHT"), Some(DisplayMode::Night));
        assert_eq!(DisplayMode::Day.cycle(), DisplayMode::Dusk);
        assert_eq!(DisplayMode::Night.cycle(), DisplayMode::Day);
    }
}
