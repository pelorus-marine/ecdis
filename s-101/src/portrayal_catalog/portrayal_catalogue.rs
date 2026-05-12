//! Opened portrayal catalogue zip bundle.

use std::io::{Cursor, Read, Seek};

use zip::ZipArchive;

use super::color_palette::ColorPalette;
use super::color_profile::{ColorProfile, parse_color_profile_xml};
use super::error::PortrayalCatalogueError;
use super::portrayal_catalogue_manifest::{
    PortrayalCatalogueManifest, parse_manifest_xml,
};

const MANIFEST_PATH: &str = "PortrayalCatalog/portrayal_catalogue.xml";
const COLOR_PROFILE_PATH: &str = "PortrayalCatalog/ColorProfiles/colorProfile.xml";

/// Opened portrayal catalogue zip bundle: manifest plus optional color profile.
#[derive(Debug, Clone)]
pub struct PortrayalCatalogue {
    /// Top-level directory inside the zip (e.g. `S-101_Portrayal-Catalogue-1.0.2`).
    pub bundle_root: String,
    pub manifest: PortrayalCatalogueManifest,
    /// Parsed `ColorProfiles/colorProfile.xml`, when present and decodable.
    pub color_profile: Option<ColorProfile>,
}

impl PortrayalCatalogue {
    /// Open an S-100 Part 9 portrayal-catalogue zip bundle.
    ///
    /// Reads the top-level directory, parses `PortrayalCatalog/portrayal_catalogue.xml`,
    /// and opportunistically parses `PortrayalCatalog/ColorProfiles/colorProfile.xml`
    /// (failure here is non-fatal — `color_profile` will be `None`).
    pub fn open_zip(bytes: &[u8]) -> Result<Self, PortrayalCatalogueError> {
        let mut archive = ZipArchive::new(Cursor::new(bytes.to_vec()))?;
        let bundle_root = find_bundle_root(&mut archive)?;

        let manifest_bytes =
            read_zip_entry(&mut archive, &format!("{bundle_root}/{MANIFEST_PATH}"))?;
        let manifest = parse_manifest_xml(&manifest_bytes)?;

        let color_profile = match read_zip_entry(
            &mut archive,
            &format!("{bundle_root}/{COLOR_PROFILE_PATH}"),
        ) {
            Ok(cp_bytes) => parse_color_profile_xml(&cp_bytes).ok(),
            Err(_) => None,
        };

        Ok(Self {
            bundle_root,
            manifest,
            color_profile,
        })
    }

    /// Convenience: look up a palette by name on the contained color profile.
    pub fn palette(&self, name: &str) -> Option<&ColorPalette> {
        self.color_profile.as_ref()?.palette(name)
    }
}

fn find_bundle_root<R: Read + Seek>(
    archive: &mut ZipArchive<R>,
) -> Result<String, PortrayalCatalogueError> {
    for i in 0..archive.len() {
        let entry = archive.by_index(i)?;
        let name = entry.name().to_string();
        if let Some(slash) = name.find('/')
            && entry.is_dir()
            && slash == name.trim_end_matches('/').len()
        {
            return Ok(name.trim_end_matches('/').to_string());
        }
    }
    // Fallback: derive from the first file entry.
    for i in 0..archive.len() {
        let entry = archive.by_index(i)?;
        let name = entry.name();
        if let Some(slash) = name.find('/') {
            return Ok(name[..slash].to_string());
        }
    }
    Err(PortrayalCatalogueError::NoBundleRoot)
}

fn read_zip_entry<R: Read + Seek>(
    archive: &mut ZipArchive<R>,
    path: &str,
) -> Result<Vec<u8>, PortrayalCatalogueError> {
    let mut file = archive
        .by_name(path)
        .map_err(|_| PortrayalCatalogueError::MissingManifest(path.to_string()))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}
