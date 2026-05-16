//! Helpers to open portrayal catalogues from disk or S-64 corpora.

use std::path::Path;
use std::sync::Arc;

use s_101::{PortrayalCatalogueBundle, PortrayalCatalogueError};

#[cfg(feature = "s64")]
use s_101::FeatureCatalogue;

/// Load a portrayal-catalogue zip from a local path.
pub fn open_portrayal_catalogue_zip(
    path: impl AsRef<Path>,
) -> Result<PortrayalCatalogueBundle, PortrayalCatalogueError> {
    let bytes: Vec<u8> = std::fs::read(path.as_ref())?;
    PortrayalCatalogueBundle::open_zip(Arc::<[u8]>::from(bytes))
}

#[cfg(feature = "s64")]
/// Open the first S-101 portrayal catalogue discovered in an IHO S-64 corpus zip.
pub fn open_s101_portrayal_from_s64_zip(
    path: impl AsRef<Path>,
) -> Result<PortrayalCatalogueBundle, PortrayalCatalogueError> {
    use s_164::Corpus;

    let mut corpus = Corpus::open(path.as_ref()).map_err(|e| PortrayalCatalogueError::Io(
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string()),
    ))?;
    let entry = corpus
        .portrayal_catalogues()
        .find(|c| c.product_identifier.as_deref() == Some("S-101"))
        .or_else(|| corpus.portrayal_catalogues().next())
        .cloned()
        .ok_or_else(|| {
            PortrayalCatalogueError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "no portrayal catalogue in S-64 zip",
            ))
        })?;
    let bytes = corpus.read_catalogue(&entry).map_err(|e| {
        PortrayalCatalogueError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    })?;
    PortrayalCatalogueBundle::open_zip(Arc::<[u8]>::from(bytes))
}

#[cfg(feature = "s64")]
/// Open an S-101 feature catalogue XML from an IHO S-64 corpus zip when present.
pub fn open_s101_feature_catalogue_from_s64_zip(
    path: impl AsRef<Path>,
) -> Result<Option<FeatureCatalogue>, PortrayalCatalogueError> {
    use s_164::Corpus;

    let mut corpus = Corpus::open(path.as_ref()).map_err(|e| PortrayalCatalogueError::Io(
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string()),
    ))?;
    let entry = corpus
        .catalogues_for_product("S-101")
        .filter(|c| !c.looks_like_portrayal_catalogue())
        .filter(|c| {
            c.file_uri
                .rsplit('/')
                .next()
                .unwrap_or(&c.file_uri)
                .to_ascii_lowercase()
                .contains("_fc")
        })
        .max_by_key(|c| c.zip_path.contains("PowerUp"))
        .cloned();
    let Some(entry) = entry else {
        return Ok(None);
    };
    let bytes = corpus.read_catalogue(&entry).map_err(|e| {
        PortrayalCatalogueError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    })?;
    FeatureCatalogue::parse_xml(&bytes).map(Some).map_err(|e| {
        PortrayalCatalogueError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            e.to_string(),
        ))
    })
}
