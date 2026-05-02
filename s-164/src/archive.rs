//! Zip corpus layout: discover `S100_ROOT/CATALOG.XML` exchange sets and read entries safely.

use std::io::{Cursor, Read, Seek};
use std::path::Path;

use zip::ZipArchive;

use crate::{S164Error, S164Result};

/// Path suffix inside each **S-100 exchange set** directory (forward slashes).
pub const S100_ROOT_CATALOG_XML_SUFFIX: &str = "S100_ROOT/CATALOG.XML";

/// Location of one exchange set inside the zip: directory that **contains** `S100_ROOT/`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExchangeSetLocation {
    /// Directory prefix ending at the parent of `S100_ROOT/` (always ends with `/`, unless empty).
    pub prefix: String,
}

impl ExchangeSetLocation {
    /// Full zip path to [`S100_ROOT_CATALOG_XML_SUFFIX`].
    #[must_use]
    pub fn catalogue_xml_path(&self) -> String {
        format!("{}{}", self.prefix, S100_ROOT_CATALOG_XML_SUFFIX)
    }
}

/// Open a zip archive stored fully in memory.
pub fn zip_archive_from_bytes(bytes: Vec<u8>) -> S164Result<ZipArchive<Cursor<Vec<u8>>>> {
    Ok(ZipArchive::new(Cursor::new(bytes))?)
}

/// Enumerate exchange sets by locating `**/S100_ROOT/CATALOG.XML` entries.
pub fn discover_exchange_sets<R: Read + Seek>(
    archive: &mut ZipArchive<R>,
) -> S164Result<Vec<ExchangeSetLocation>> {
    let mut prefixes = Vec::new();
    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        let name = normalize_slashes(file.name());
        if let Some(prefix) = name.strip_suffix(S100_ROOT_CATALOG_XML_SUFFIX) {
            prefixes.push(ExchangeSetLocation {
                prefix: normalize_zip_prefix(prefix),
            });
        }
    }
    prefixes.sort();
    prefixes.dedup();
    Ok(prefixes)
}

fn normalize_slashes(path: &str) -> String {
    path.replace('\\', "/")
}

fn normalize_zip_prefix(prefix: &str) -> String {
    let p = prefix.trim_end_matches('/');
    if p.is_empty() {
        String::new()
    } else {
        format!("{p}/")
    }
}

/// Read a zip member into memory after normalizing separators.
pub fn read_zip_entry<R: Read + Seek>(
    archive: &mut ZipArchive<R>,
    path: &str,
) -> S164Result<Vec<u8>> {
    let normalized = normalize_slashes(path);
    let mut file = archive
        .by_name(&normalized)
        .map_err(|_| S164Error::MissingZipEntry(normalized.clone()))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

/// Join [`ExchangeSetLocation::prefix`] with a `file:/…` URI from dataset discovery metadata.
///
/// [`DatasetDiscovery::file_uri`](crate::DatasetDiscovery::file_uri) paths are relative to
/// **`S100_ROOT`**. For example `file:/S-101/DATASET_FILES/cell.000` resolves to
/// `{prefix}S100_ROOT/S-101/DATASET_FILES/cell.000` inside the archive.
pub fn resolve_bundle_path(exchange_parent_prefix: &str, file_uri: &str) -> S164Result<String> {
    let rest = file_uri
        .strip_prefix("file:")
        .ok_or_else(|| S164Error::InvalidFileUri(file_uri.to_string()))?;
    let rest = rest.trim_start_matches('/');
    refuse_traversal(exchange_parent_prefix)?;
    refuse_traversal(rest)?;
    let parent = exchange_parent_prefix.trim_end_matches('/');
    let joined = if parent.is_empty() {
        format!("S100_ROOT/{rest}")
    } else {
        format!("{parent}/S100_ROOT/{rest}")
    };
    refuse_traversal(&joined)?;
    Ok(normalize_slashes(&joined))
}

fn refuse_traversal(path: &str) -> S164Result<()> {
    if path.is_empty() {
        return Ok(());
    }
    for c in Path::new(path).components() {
        use std::path::Component::*;
        match c {
            Normal(_) => {}
            CurDir => {}
            ParentDir | RootDir | Prefix(_) => {
                return Err(S164Error::PathTraversal(path.to_string()));
            }
        }
    }
    Ok(())
}
