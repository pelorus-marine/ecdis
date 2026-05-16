//! Error variants for portrayal-catalogue loading.

#[derive(Debug, thiserror::Error)]
pub enum PortrayalCatalogueError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),

    #[error("portrayal catalogue bundle has no top-level directory")]
    NoBundleRoot,

    #[error("missing manifest entry in bundle: {0}")]
    MissingManifest(String),

    #[error("manifest XML decode error: {0}")]
    ManifestXml(#[from] roxmltree::Error),

    #[error("manifest XML is not valid UTF-8")]
    ManifestNotUtf8,

    #[error("missing <portrayalCatalog> root element")]
    MissingManifestRoot,

    #[error("color profile XML decode error: {0}")]
    ColorProfileXml(roxmltree::Error),

    #[error("color profile XML is not valid UTF-8")]
    ColorProfileNotUtf8,

    #[error("missing asset in bundle: {0}")]
    MissingAsset(String),

    #[error("unknown symbol id: {0}")]
    UnknownSymbol(String),

    #[error("palette has no css attribute")]
    PaletteHasNoStylesheet,
}
