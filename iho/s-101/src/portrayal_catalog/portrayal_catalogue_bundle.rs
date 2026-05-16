//! Zip-backed portrayal catalogue with on-demand asset reads.

use std::io::{Cursor, Read, Seek, Write};
use std::sync::Arc;

use zip::ZipArchive;

use super::color_palette::ColorPalette;
use super::error::PortrayalCatalogueError;
use super::named_asset::NamedAsset;
use super::portrayal_catalogue::PortrayalCatalogue;

const SYMBOLS_DIR: &str = "PortrayalCatalog/Symbols";
const COLOR_PROFILES_DIR: &str = "PortrayalCatalog/ColorProfiles";

/// Parsed catalogue metadata plus retained zip bytes for symbol / stylesheet assets.
#[derive(Debug, Clone)]
pub struct PortrayalCatalogueBundle {
    pub catalogue: PortrayalCatalogue,
    zip: Arc<[u8]>,
}

impl PortrayalCatalogueBundle {
    /// Parse manifest and colour profile, retaining `bytes` for later asset reads.
    pub fn open_zip(bytes: impl Into<Arc<[u8]>>) -> Result<Self, PortrayalCatalogueError> {
        let zip = bytes.into();
        let catalogue = PortrayalCatalogue::open_zip(&zip)?;
        Ok(Self { catalogue, zip })
    }

    #[must_use]
    pub fn zip_bytes(&self) -> &[u8] {
        &self.zip
    }

    /// Read an arbitrary path inside the bundle (forward slashes).
    pub fn read_entry(&self, relative_path: &str) -> Result<Vec<u8>, PortrayalCatalogueError> {
        let path = format!("{}/{}", self.catalogue.bundle_root, relative_path);
        with_archive(&self.zip, |archive| read_zip_entry(archive, &path))
    }

    /// Raw SVG bytes for a manifest symbol `id` (e.g. `ACHARE02`).
    pub fn read_symbol_svg(&self, symbol_id: &str) -> Result<Vec<u8>, PortrayalCatalogueError> {
        let asset = self
            .catalogue
            .manifest
            .symbols
            .iter()
            .find(|s| s.id == symbol_id)
            .ok_or_else(|| PortrayalCatalogueError::UnknownSymbol(symbol_id.to_string()))?;
        let file_name = asset
            .file_name
            .as_deref()
            .ok_or_else(|| PortrayalCatalogueError::UnknownSymbol(symbol_id.to_string()))?;
        self.read_entry(&format!("{SYMBOLS_DIR}/{file_name}"))
    }

    /// Stylesheet referenced by `<palette css="…">` when present.
    ///
    /// IHO S-101 catalogues ship symbol CSS under `PortrayalCatalog/Symbols/` (e.g.
    /// `daySvgStyle.css` with `.sCHBLK` / `.fCHWHT` rules). Older paths under
    /// `ColorProfiles/` are tried as a fallback.
    pub fn read_palette_stylesheet(
        &self,
        palette: &ColorPalette,
    ) -> Result<Vec<u8>, PortrayalCatalogueError> {
        let css = palette
            .css
            .as_deref()
            .ok_or(PortrayalCatalogueError::PaletteHasNoStylesheet)?;
        let symbols_path = format!("{SYMBOLS_DIR}/{css}");
        match self.read_entry(&symbols_path) {
            Ok(bytes) => Ok(bytes),
            Err(_) => self.read_entry(&format!("{COLOR_PROFILES_DIR}/{css}")),
        }
    }

    /// Look up a symbol [`NamedAsset`] by id.
    #[must_use]
    pub fn symbol_asset(&self, symbol_id: &str) -> Option<&NamedAsset> {
        self.catalogue.manifest.symbols.iter().find(|s| s.id == symbol_id)
    }

    /// Palette names declared in `colorProfile.xml`.
    pub fn palette_names(&self) -> Vec<&str> {
        self.catalogue
            .color_profile
            .as_ref()
            .map(|cp| cp.palettes.iter().map(|p| p.name.as_str()).collect())
            .unwrap_or_default()
    }
}

fn with_archive<T, F>(zip: &[u8], f: F) -> Result<T, PortrayalCatalogueError>
where
    F: FnOnce(&mut ZipArchive<Cursor<&[u8]>>) -> Result<T, PortrayalCatalogueError>,
{
    let mut archive = ZipArchive::new(Cursor::new(zip))?;
    f(&mut archive)
}

fn read_zip_entry<R: Read + Seek>(
    archive: &mut ZipArchive<R>,
    path: &str,
) -> Result<Vec<u8>, PortrayalCatalogueError> {
    let mut file = archive
        .by_name(path)
        .map_err(|_| PortrayalCatalogueError::MissingAsset(path.to_string()))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

/// Fallback CSS when the catalogue's `*SvgStyle.css` file is missing.
///
/// S-101 symbol SVG uses `class="sCHBLK"` / `class="fCHWHT"` (not bare token names).
pub fn stylesheet_from_palette(palette: &ColorPalette) -> Vec<u8> {
    let mut out = Vec::new();
    let _ = writeln!(out, "/* generated from colorProfile palette \"{}\" */", palette.name);
    let _ = writeln!(out, ".layout {{ display: none; }}");
    let _ = writeln!(out, ".f0 {{ fill: none; }}");
    for item in &palette.items {
        let (r, g, b) = item.srgb;
        let hex = format!("#{r:02x}{g:02x}{b:02x}");
        let _ = writeln!(out, ".s{} {{ stroke: {hex}; }}", item.token);
        let _ = writeln!(out, ".f{} {{ fill: {hex}; }}", item.token);
    }
    out
}
