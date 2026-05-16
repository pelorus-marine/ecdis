/// Errors produced when constructing or driving a [`super::CatalogueBackedPortrayal`].
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
