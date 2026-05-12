//! Minimal parsing of **S-100 exchange catalogue** (`CATALOG.XML`) files found in S-164 bundles.

use roxmltree::Document;

use crate::{S164Error, S164Result};

/// Parsed subset of **S100_ExchangeCatalogue** needed for tooling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExchangeCatalogue {
    /// Inner catalogue name (e.g. `DisplayStandard`), not the issuing agency block.
    pub catalogue_identifier: String,
    /// Dataset rows from `datasetDiscoveryMetadata` (ENC cell `.000`, updates, etc.).
    pub datasets: Vec<DatasetDiscovery>,
    /// Catalogue rows from `catalogueDiscoveryMetadata` (feature / portrayal / alert catalogues).
    pub catalogues: Vec<CatalogueDiscovery>,
}

/// One **S100_DatasetDiscoveryMetadata** block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetDiscovery {
    pub file_uri: String,
    pub product_identifier: Option<String>,
}

/// One **S100_CatalogueDiscoveryMetadata** block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogueDiscovery {
    pub file_uri: String,
    pub product_identifier: Option<String>,
    /// Catalogue scope as advertised by the issuer (`featureCatalogue`, `portrayalCatalogue`, …).
    /// Note: IHO S-164 v1.2.0 labels portrayal catalogues as `featureCatalogue`; combine with
    /// [`compressed`](Self::compressed) and filename to distinguish.
    pub scope: Option<String>,
    /// Whether the file is a zip bundle (Part 9 portrayal catalogues are typically `true`).
    pub compressed: Option<bool>,
}

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
