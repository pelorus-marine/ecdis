use std::io::Cursor;

use zip::ZipArchive;

use crate::archive::{
    ExchangeSetLocation, discover_exchange_sets, read_zip_entry, resolve_bundle_path,
};
use crate::catalogue::parse_exchange_catalogue;
use crate::{S164Error, S164Result};

use super::{
    CatalogueEntry, Classification, DatasetEntry, ExchangeSetEntry,
};

pub(super) fn build_index(
    archive: &mut ZipArchive<Cursor<Vec<u8>>>,
) -> S164Result<(
    Vec<ExchangeSetEntry>,
    Vec<DatasetEntry>,
    Vec<CatalogueEntry>,
)> {
    let locations = discover_exchange_sets(archive)?;
    let mut exchange_sets = Vec::with_capacity(locations.len());
    let mut datasets = Vec::new();
    let mut catalogues = Vec::new();

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
        for cat in catalogue.catalogues {
            let zip_path = match resolve_bundle_path(&location.prefix, &cat.file_uri) {
                Ok(p) => p,
                Err(S164Error::PathTraversal(_) | S164Error::InvalidFileUri(_)) => continue,
                Err(e) => return Err(e),
            };
            catalogues.push(CatalogueEntry {
                exchange_set_index: index,
                product_identifier: cat.product_identifier,
                file_uri: cat.file_uri,
                zip_path,
                scope: cat.scope,
                compressed: cat.compressed,
                classification,
            });
        }
    }

    Ok((exchange_sets, datasets, catalogues))
}

fn load_catalogue(
    archive: &mut ZipArchive<Cursor<Vec<u8>>>,
    location: &ExchangeSetLocation,
) -> S164Result<crate::catalogue::ExchangeCatalogue> {
    let xml = read_zip_entry(archive, &location.catalogue_xml_path())?;
    parse_exchange_catalogue(&xml)
}
