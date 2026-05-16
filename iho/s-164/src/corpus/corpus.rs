use std::io::Cursor;
use std::path::Path;

use zip::ZipArchive;

use crate::archive::{read_zip_entry, zip_archive_from_bytes};
use crate::download::{
    DEFAULT_TEST_DATA_ZIP_V1_2_0_SHA256, DEFAULT_TEST_DATA_ZIP_V1_2_0_URL, cached_download,
};
use crate::{S164Result};

use super::build::build_index;
use super::{CatalogueEntry, DatasetEntry, ExchangeSetEntry};

/// Opened S-164 corpus: zip archive plus precomputed exchange-set / dataset / catalogue indexes.
pub struct Corpus {
    archive: ZipArchive<Cursor<Vec<u8>>>,
    exchange_sets: Vec<ExchangeSetEntry>,
    datasets: Vec<DatasetEntry>,
    catalogues: Vec<CatalogueEntry>,
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
        let (exchange_sets, datasets, catalogues) = build_index(&mut archive)?;
        Ok(Self {
            archive,
            exchange_sets,
            datasets,
            catalogues,
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

    /// All catalogue artifacts discovered across every exchange set (FC / portrayal / alert).
    #[must_use]
    pub fn catalogues(&self) -> &[CatalogueEntry] {
        &self.catalogues
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

    /// Catalogue rows whose `productIdentifier` exactly matches `product_id`.
    pub fn catalogues_for_product<'a>(
        &'a self,
        product_id: &'a str,
    ) -> impl Iterator<Item = &'a CatalogueEntry> + 'a {
        self.catalogues
            .iter()
            .filter(move |c| c.product_identifier.as_deref() == Some(product_id))
    }

    /// Iterator over catalogue entries that **look like portrayal catalogues**
    /// (filename contains `Portrayal`).
    pub fn portrayal_catalogues(&self) -> impl Iterator<Item = &CatalogueEntry> + '_ {
        self.catalogues.iter().filter(|c| c.looks_like_portrayal_catalogue())
    }

    /// Read raw bytes for a dataset entry from the underlying zip.
    pub fn read_dataset(&mut self, entry: &DatasetEntry) -> S164Result<Vec<u8>> {
        read_zip_entry(&mut self.archive, &entry.zip_path)
    }

    /// Read raw bytes for a catalogue entry from the underlying zip.
    pub fn read_catalogue(&mut self, entry: &CatalogueEntry) -> S164Result<Vec<u8>> {
        read_zip_entry(&mut self.archive, &entry.zip_path)
    }
}
