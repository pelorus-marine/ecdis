//! High-level corpus abstraction over an S-164 zip bundle.
//!
//! [`Corpus::fetch_default`] downloads, caches, and verifies the IHO **v1.2.0** asset.
//! [`Corpus::open`] / [`Corpus::from_bytes`] cover the offline cases. Once constructed,
//! the corpus exposes precomputed [`ExchangeSetEntry`] / [`DatasetEntry`] indexes and
//! reads raw dataset bytes from the archive on demand.

use std::io::Cursor;
use std::path::Path;

use zip::ZipArchive;

use crate::archive::{
    ExchangeSetLocation, discover_exchange_sets, read_zip_entry, resolve_bundle_path,
    zip_archive_from_bytes,
};
use crate::catalogue::parse_exchange_catalogue;
use crate::download::{
    DEFAULT_TEST_DATA_ZIP_V1_2_0_SHA256, DEFAULT_TEST_DATA_ZIP_V1_2_0_URL, cached_download,
};
use crate::{S164Error, S164Result};

/// How the IHO test corpus uses an exchange set, derived from its directory prefix.
///
/// The IHO conformance manual scopes negative tests by directory naming; this enum
/// captures the layer at which a negative test is designed to fail. Positive cases
/// are the default for any prefix not listed below.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Classification {
    /// Standard dataset, expected to load cleanly through every layer.
    Positive,
    /// Byte-level malformed dataset (e.g. `CorruptData/…`). ISO 8211 parsers must reject.
    NegativeBytes,
    /// Bytes parse, but S-101 update-sequence rules are violated (e.g. `InvalidSequence00N/…`).
    NegativeUpdateSequence,
    /// Recognised exchange set whose role is not a parse-failure scenario.
    Other,
}

impl Classification {
    /// Classify by exchange-set prefix (path inside the zip ending in `/`).
    #[must_use]
    pub fn from_exchange_set_prefix(prefix: &str) -> Self {
        if prefix.contains("CorruptData") {
            Self::NegativeBytes
        } else if prefix.contains("InvalidSequence") {
            Self::NegativeUpdateSequence
        } else {
            Self::Positive
        }
    }

    /// `true` when the corpus author expects parsing to fail at the ISO 8211 layer.
    #[must_use]
    pub fn expects_iso8211_parse_failure(self) -> bool {
        matches!(self, Self::NegativeBytes)
    }
}

/// One exchange set discovered in the corpus zip.
#[derive(Debug, Clone)]
pub struct ExchangeSetEntry {
    /// Zip-prefix ending at the parent of `S100_ROOT/` (always ends with `/`, unless empty).
    pub prefix: String,
    /// Inner catalogue identifier from `<identifier>` (e.g. `DisplayStandard`).
    pub catalogue_identifier: String,
    /// Scenario classification derived from [`prefix`](Self::prefix).
    pub classification: Classification,
}

/// One dataset row from an exchange catalogue, with its resolved zip path.
#[derive(Debug, Clone)]
pub struct DatasetEntry {
    /// Index into [`Corpus::exchange_sets`].
    pub exchange_set_index: usize,
    /// `productIdentifier` from the catalogue (`Some("S-101")`, `Some("S-102")`, etc.).
    pub product_identifier: Option<String>,
    /// `fileName` URI from the catalogue (`file:/…`).
    pub file_uri: String,
    /// Fully resolved entry path inside the zip (input to [`Corpus::read_dataset`]).
    pub zip_path: String,
    /// Classification copied from the owning exchange set for convenience.
    pub classification: Classification,
}

/// Opened S-164 corpus: zip archive plus precomputed exchange-set / dataset indexes.
pub struct Corpus {
    archive: ZipArchive<Cursor<Vec<u8>>>,
    exchange_sets: Vec<ExchangeSetEntry>,
    datasets: Vec<DatasetEntry>,
}

impl Corpus {
    /// Download the default IHO **v1.2.0** corpus (cached, SHA-256 verified) and open it.
    ///
    /// See [`DEFAULT_TEST_DATA_ZIP_V1_2_0_URL`] and [`DEFAULT_TEST_DATA_ZIP_V1_2_0_SHA256`].
    pub fn fetch_default() -> S164Result<Self> {
        Self::fetch(
            DEFAULT_TEST_DATA_ZIP_V1_2_0_URL,
            Some(DEFAULT_TEST_DATA_ZIP_V1_2_0_SHA256),
        )
    }

    /// Download from a custom URL with caching and optional digest verification.
    pub fn fetch(url: &str, expected_sha256: Option<&str>) -> S164Result<Self> {
        let bytes = cached_download(url, expected_sha256)?;
        Self::from_bytes(bytes)
    }

    /// Open a corpus zip from a local path. No verification is performed.
    pub fn open(path: impl AsRef<Path>) -> S164Result<Self> {
        let bytes = std::fs::read(path.as_ref())?;
        Self::from_bytes(bytes)
    }

    /// Construct from in-memory zip bytes. No verification is performed.
    pub fn from_bytes(bytes: Vec<u8>) -> S164Result<Self> {
        let mut archive = zip_archive_from_bytes(bytes)?;
        let (exchange_sets, datasets) = build_index(&mut archive)?;
        Ok(Self {
            archive,
            exchange_sets,
            datasets,
        })
    }

    /// All exchange sets discovered in the corpus, sorted by zip prefix.
    #[must_use]
    pub fn exchange_sets(&self) -> &[ExchangeSetEntry] {
        &self.exchange_sets
    }

    /// All datasets discovered across every exchange set.
    #[must_use]
    pub fn datasets(&self) -> &[DatasetEntry] {
        &self.datasets
    }

    /// Iterator over datasets whose catalogue advertises `productIdentifier == product_id`.
    pub fn datasets_for_product<'a>(
        &'a self,
        product_id: &'a str,
    ) -> impl Iterator<Item = &'a DatasetEntry> + 'a {
        self.datasets
            .iter()
            .filter(move |d| d.product_identifier.as_deref() == Some(product_id))
    }

    /// Read raw bytes for a dataset entry from the underlying zip.
    pub fn read_dataset(&mut self, entry: &DatasetEntry) -> S164Result<Vec<u8>> {
        read_zip_entry(&mut self.archive, &entry.zip_path)
    }
}

fn build_index(
    archive: &mut ZipArchive<Cursor<Vec<u8>>>,
) -> S164Result<(Vec<ExchangeSetEntry>, Vec<DatasetEntry>)> {
    let locations = discover_exchange_sets(archive)?;
    let mut exchange_sets = Vec::with_capacity(locations.len());
    let mut datasets = Vec::new();

    for (index, location) in locations.iter().enumerate() {
        let catalogue = load_catalogue(archive, location)?;
        let classification = Classification::from_exchange_set_prefix(&location.prefix);
        exchange_sets.push(ExchangeSetEntry {
            prefix: location.prefix.clone(),
            catalogue_identifier: catalogue.catalogue_identifier,
            classification,
        });
        for ds in catalogue.datasets {
            let zip_path = match resolve_bundle_path(&location.prefix, &ds.file_uri) {
                Ok(p) => p,
                Err(S164Error::PathTraversal(_) | S164Error::InvalidFileUri(_)) => continue,
                Err(e) => return Err(e),
            };
            datasets.push(DatasetEntry {
                exchange_set_index: index,
                product_identifier: ds.product_identifier,
                file_uri: ds.file_uri,
                zip_path,
                classification,
            });
        }
    }

    Ok((exchange_sets, datasets))
}

fn load_catalogue(
    archive: &mut ZipArchive<Cursor<Vec<u8>>>,
    location: &ExchangeSetLocation,
) -> S164Result<crate::catalogue::ExchangeCatalogue> {
    let xml = read_zip_entry(archive, &location.catalogue_xml_path())?;
    parse_exchange_catalogue(&xml)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classification_known_prefixes() {
        assert_eq!(
            Classification::from_exchange_set_prefix("S-100/CorruptData/"),
            Classification::NegativeBytes
        );
        assert_eq!(
            Classification::from_exchange_set_prefix("S-100/InvalidSequence001/"),
            Classification::NegativeUpdateSequence
        );
        assert_eq!(
            Classification::from_exchange_set_prefix("S-100/DisplayStandard/"),
            Classification::Positive
        );
    }

    #[test]
    fn only_negative_bytes_expects_iso8211_failure() {
        assert!(Classification::NegativeBytes.expects_iso8211_parse_failure());
        assert!(!Classification::NegativeUpdateSequence.expects_iso8211_parse_failure());
        assert!(!Classification::Positive.expects_iso8211_parse_failure());
        assert!(!Classification::Other.expects_iso8211_parse_failure());
    }
}
