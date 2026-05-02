//! IHO **S-164** — helpers for **published test corpora** (zip bundles of S-100 exchange sets).
//!
//! This crate downloads released archives (for example from the IHO-OHI GitHub project) and
//! performs **structural** discovery: locate `S100_ROOT/CATALOG.XML`, parse a minimal view of
//! dataset discovery metadata. Full ENC semantics remain in [`s_101`](https://crates.io/crates/s-101)
//! and related product crates.
//!
//! ### Runnable examples
//!
//! ```bash
//! cargo run -p s-164 --example inventory -- local ./S-64_1.2.0.zip
//! cargo run -p s-164 --example inventory -- download
//! cargo run -p s-164 --example parse_catalog_xml -- ./CATALOG.XML
//! ```
//!
//! See **`examples/`** in the crate directory and the **Examples** section in `README.md`.

#![forbid(unsafe_code)]

mod archive;
mod catalogue;
mod download;
mod error;

pub use archive::{
    ExchangeSetLocation, S100_ROOT_CATALOG_XML_SUFFIX, discover_exchange_sets, read_zip_entry,
    resolve_bundle_path, zip_archive_from_bytes,
};
pub use catalogue::{DatasetDiscovery, ExchangeCatalogue, parse_exchange_catalogue};
pub use download::{DEFAULT_TEST_DATA_ZIP_V1_2_0_URL, download_bytes, download_bytes_with_timeout};
pub use error::{S164Error, S164Result};

use std::io::{Read, Seek};

use zip::ZipArchive;

/// Load UTF-8 `CATALOG.XML` bytes for [`ExchangeSetLocation`] and parse [`ExchangeCatalogue`].
pub fn load_exchange_catalogue<R: Read + Seek>(
    archive: &mut ZipArchive<R>,
    location: &ExchangeSetLocation,
) -> S164Result<ExchangeCatalogue> {
    let xml = read_zip_entry(archive, &location.catalogue_xml_path())?;
    parse_exchange_catalogue(&xml)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL_CATALOGUE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<S100XC:S100_ExchangeCatalogue xmlns:S100XC="http://www.iho.int/s100/xc/5.0">
  <S100XC:identifier>
    <S100XC:identifier>Demo</S100XC:identifier>
  </S100XC:identifier>
  <S100XC:datasetDiscoveryMetadata>
    <S100XC:S100_DatasetDiscoveryMetadata>
      <S100XC:fileName>file:/S-101/DATASET_FILES/x.000</S100XC:fileName>
      <S100XC:productSpecification>
        <S100XC:productIdentifier>S-101</S100XC:productIdentifier>
      </S100XC:productSpecification>
    </S100XC:S100_DatasetDiscoveryMetadata>
  </S100XC:datasetDiscoveryMetadata>
</S100XC:S100_ExchangeCatalogue>"#;

    #[test]
    fn parses_minimal_exchange_catalogue() {
        let parsed = parse_exchange_catalogue(MINIMAL_CATALOGUE.as_bytes()).unwrap();
        assert_eq!(parsed.catalogue_identifier, "Demo");
        assert_eq!(parsed.datasets.len(), 1);
        assert_eq!(
            parsed.datasets[0].file_uri,
            "file:/S-101/DATASET_FILES/x.000"
        );
        assert_eq!(
            parsed.datasets[0].product_identifier.as_deref(),
            Some("S-101")
        );
    }

    #[test]
    fn resolves_dataset_paths_under_exchange_prefix() {
        let p = resolve_bundle_path(
            "S-100/DisplayStandard/",
            "file:/S-101/DATASET_FILES/10100AA_STNDR.000",
        )
        .unwrap();
        assert_eq!(
            p,
            "S-100/DisplayStandard/S100_ROOT/S-101/DATASET_FILES/10100AA_STNDR.000"
        );
    }

    #[test]
    fn rejects_parent_dir_in_resolved_paths() {
        assert!(resolve_bundle_path("pre/", "file:/../etc/passwd").is_err());
    }

    #[test]
    fn discovers_exchange_sets_in_official_zip_when_present() {
        let zip_path = std::path::Path::new("/tmp/S-64_1.2.0.zip");
        if !zip_path.exists() {
            return;
        }
        let file = std::fs::File::open(zip_path).unwrap();
        let mut archive = ZipArchive::new(file).unwrap();
        let sets = discover_exchange_sets(&mut archive).unwrap();
        assert!(
            sets.iter().any(|s| s.prefix.contains("DisplayStandard")),
            "{sets:?}"
        );

        let loc = sets.iter().find(|s| s.prefix.contains("DisplayStandard")).unwrap();
        let cat = load_exchange_catalogue(&mut archive, loc).unwrap();
        assert_eq!(cat.catalogue_identifier, "DisplayStandard");
        assert!(!cat.datasets.is_empty());

        let ds = &cat.datasets[0];
        let rel = resolve_bundle_path(&loc.prefix, &ds.file_uri).unwrap();
        let bytes = read_zip_entry(&mut archive, &rel).unwrap();
        assert!(bytes.len() > 100);
    }

    #[test]
    #[ignore = "network: downloads ~6 MB GitHub release asset"]
    fn fetch_default_bundle_smoke() {
        let bytes = download_bytes(DEFAULT_TEST_DATA_ZIP_V1_2_0_URL).unwrap();
        let mut archive = zip_archive_from_bytes(bytes).unwrap();
        let sets = discover_exchange_sets(&mut archive).unwrap();
        assert!(sets.len() > 5);
    }
}
