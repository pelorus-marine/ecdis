use thiserror::Error;

/// Failure downloading, reading, or parsing IHO S-164 packaged test material.
#[derive(Debug, Error)]
pub enum S164Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("HTTP request failed: {0}")]
    Http(String),

    #[error("HTTP error fetching {url}: status {status}")]
    HttpStatus { url: String, status: u16 },

    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),

    #[error(transparent)]
    XmlDom(#[from] roxmltree::Error),

    #[error("exchange catalogue XML is not valid UTF-8")]
    CatalogueNotUtf8,

    #[error("missing S100_ExchangeCatalogue root element")]
    MissingExchangeCatalogueRoot,

    #[error("invalid dataset file URI (expected file:/…): {0}")]
    InvalidFileUri(String),

    #[error("refused potentially unsafe path: {0}")]
    PathTraversal(String),

    #[error("missing zip entry: {0}")]
    MissingZipEntry(String),

    #[error("could not determine cache directory (set S164_CACHE_DIR)")]
    CacheDirUnavailable,

    #[error("could not derive cache filename from URL: {0}")]
    CacheFilenameFromUrl(String),

    #[error("SHA-256 mismatch for {what}: expected {expected}, got {actual}")]
    Sha256Mismatch {
        what: String,
        expected: String,
        actual: String,
    },
}

pub type S164Result<T> = Result<T, S164Error>;
