//! Minimal parsing of **S-100 exchange catalogue** (`CATALOG.XML`) files found in S-164 bundles.
//!
//! Each public type lives in its own file under `catalogue/`; this `mod.rs` is just
//! the namespace assembly point.

mod catalogue_discovery;
mod dataset_discovery;
mod exchange_catalogue;

pub use catalogue_discovery::CatalogueDiscovery;
pub use dataset_discovery::DatasetDiscovery;
pub use exchange_catalogue::ExchangeCatalogue;

use roxmltree::Document;

use crate::{S164Error, S164Result};

/// Parse exchange catalogue XML (UTF-8).
pub fn parse_exchange_catalogue(xml: &[u8]) -> S164Result<ExchangeCatalogue> {
    let text = std::str::from_utf8(xml).map_err(|_| S164Error::CatalogueNotUtf8)?;
    let doc = Document::parse(text)?;
    let root = doc
        .descendants()
        .find(|n| n.tag_name().name() == "S100_ExchangeCatalogue")
        .ok_or(S164Error::MissingExchangeCatalogueRoot)?;

    let catalogue_identifier = root
        .children()
        .find(|n| n.tag_name().name() == "identifier")
        .and_then(|id_el| id_el.children().find(|n| n.tag_name().name() == "identifier"))
        .and_then(|n| n.text())
        .unwrap_or("")
        .trim()
        .to_string();

    let mut datasets = Vec::new();
    for ddm in root
        .descendants()
        .filter(|n| n.tag_name().name() == "S100_DatasetDiscoveryMetadata")
    {
        let Some(file_uri) = ddm
            .children()
            .find(|n| n.tag_name().name() == "fileName")
            .and_then(|n| n.text())
            .map(str::trim)
            .filter(|s| !s.is_empty())
        else {
            continue;
        };
        let product_identifier = ddm
            .descendants()
            .find(|n| n.tag_name().name() == "productIdentifier")
            .and_then(|n| n.text())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());
        datasets.push(DatasetDiscovery {
            file_uri: file_uri.to_string(),
            product_identifier,
        });
    }

    let mut catalogues = Vec::new();
    for cdm in root
        .descendants()
        .filter(|n| n.tag_name().name() == "S100_CatalogueDiscoveryMetadata")
    {
        let Some(file_uri) = cdm
            .children()
            .find(|n| n.tag_name().name() == "fileName")
            .and_then(|n| n.text())
            .map(str::trim)
            .filter(|s| !s.is_empty())
        else {
            continue;
        };
        let product_identifier = cdm
            .descendants()
            .find(|n| n.tag_name().name() == "productIdentifier")
            .and_then(|n| n.text())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());
        let scope = cdm
            .children()
            .find(|n| n.tag_name().name() == "scope")
            .and_then(|n| n.text())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());
        let compressed = cdm
            .children()
            .find(|n| n.tag_name().name() == "compressionFlag")
            .and_then(|n| n.text())
            .map(str::trim)
            .and_then(|s| match s {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            });
        catalogues.push(CatalogueDiscovery {
            file_uri: file_uri.to_string(),
            product_identifier,
            scope,
            compressed,
        });
    }

    Ok(ExchangeCatalogue {
        catalogue_identifier,
        datasets,
        catalogues,
    })
}
