//! **S-100 Part 9 Portrayal Catalogue** loader for S-101 (manifest + color profile only).
//!
//! Portrayal catalogues ship as **zip bundles**: at the top a single directory
//! `S-101_Portrayal-Catalogue-<version>/` contains `FeatureCatalog.xml` (a copy of the FC)
//! and `PortrayalCatalog/` with `portrayal_catalogue.xml` (the manifest),
//! `ColorProfiles/colorProfile.xml`, `AlertCatalog-S101.xml`, plus directories of
//! `Symbols/*.svg`, `LineStyles/*.xml`, `AreaFills/*.xml`, and `Rules/*.lua`.
//!
//! This module parses **the manifest and color profile only**. SVG / line-style / area-fill /
//! rule (Lua) content is intentionally **not** parsed or executed here — that work belongs to
//! a downstream portrayal pipeline.
//!
//! Each public type lives in its own file under `portrayal_catalog/`; this `mod.rs` is just
//! the namespace assembly point.

mod color_palette;
mod color_palette_item;
mod color_profile;
mod color_token_decl;
mod error;
mod named_asset;
mod portrayal_catalogue;
mod portrayal_catalogue_bundle;
mod portrayal_catalogue_manifest;
mod rule_asset;
mod xml_util;

pub use color_palette::ColorPalette;
pub use color_palette_item::ColorPaletteItem;
pub use color_profile::{ColorProfile, parse_color_profile_xml};
pub use color_token_decl::ColorTokenDecl;
pub use error::PortrayalCatalogueError;
pub use named_asset::NamedAsset;
pub use portrayal_catalogue::PortrayalCatalogue;
pub use portrayal_catalogue_bundle::{PortrayalCatalogueBundle, stylesheet_from_palette};
pub use portrayal_catalogue_manifest::{PortrayalCatalogueManifest, parse_manifest_xml};
pub use rule_asset::RuleAsset;

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL_MANIFEST: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<portrayalCatalog productId="S-101" version="1.0.2">
  <alertCatalog id="alertCatalog">
    <fileName>AlertCatalog-S101.xml</fileName>
    <fileType>AlertCatalog</fileType>
  </alertCatalog>
  <colorProfiles>
    <colorProfile id="1">
      <fileName>colorProfile.xml</fileName>
    </colorProfile>
  </colorProfiles>
  <symbols>
    <symbol id="ACHARE02"><fileName>ACHARE02.svg</fileName><fileFormat>SVG</fileFormat></symbol>
    <symbol id="BOYMOR02"><fileName>BOYMOR02.svg</fileName><fileFormat>SVG</fileFormat></symbol>
  </symbols>
  <lineStyles>
    <lineStyle id="ACHARE51"><fileName>ACHARE51.xml</fileName></lineStyle>
  </lineStyles>
  <areaFills>
    <areaFill id="DRGARE01"><fileName>DRGARE01.xml</fileName></areaFill>
  </areaFills>
  <rules>
    <ruleFile id="main"><fileType>Rule</fileType><ruleType>TopLevelTemplate</ruleType></ruleFile>
    <ruleFile id="AnchorageArea"><fileType>Rule</fileType><ruleType>SubTemplate</ruleType></ruleFile>
    <ruleFile id="Default"><fileType>Rule</fileType><ruleType>SubTemplate</ruleType></ruleFile>
  </rules>
</portrayalCatalog>"#;

    const MINIMAL_COLOR_PROFILE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<colorProfile>
  <colors>
    <color token="CHRED" name="red"><description>Chart red</description></color>
  </colors>
  <palette name="Day" css="daySvgStyle.css">
    <item token="CHRED">
      <cie><xyL><x>0.55</x><y>0.30</y><L>30</L></xyL></cie>
      <srgb><red>255</red><green>0</green><blue>0</blue></srgb>
    </item>
  </palette>
  <palette name="Night">
    <item token="CHRED">
      <srgb><red>96</red><green>0</green><blue>0</blue></srgb>
    </item>
  </palette>
</colorProfile>"#;

    #[test]
    fn manifest_round_trip() {
        let m = parse_manifest_xml(MINIMAL_MANIFEST.as_bytes()).unwrap();
        assert_eq!(m.product_id, "S-101");
        assert_eq!(m.version, "1.0.2");
        assert_eq!(
            m.alert_catalog_file.as_deref(),
            Some("AlertCatalog-S101.xml")
        );
        assert_eq!(m.color_profile_files, vec!["colorProfile.xml"]);
        assert_eq!(m.symbols.len(), 2);
        assert_eq!(m.symbols[0].id, "ACHARE02");
        assert_eq!(m.symbols[0].file_name.as_deref(), Some("ACHARE02.svg"));
        assert_eq!(m.line_styles.len(), 1);
        assert_eq!(m.area_fills.len(), 1);
        assert_eq!(m.rules.len(), 3);
        assert!(m.rules.iter().any(|r| r.is_top_level() && r.id == "main"));
        assert!(m.rules.iter().any(|r| r.is_sub_template() && r.id == "AnchorageArea"));
    }

    #[test]
    fn opens_real_bundle_when_extracted_zip_present() {
        let p = std::path::Path::new("/tmp/pc-1.0.2.zip");
        if !p.exists() {
            return;
        }
        let bytes = std::fs::read(p).unwrap();
        let pc = PortrayalCatalogue::open_zip(&bytes).unwrap();
        assert!(pc.bundle_root.contains("Portrayal-Catalogue"));
        assert_eq!(pc.manifest.product_id, "S-101");
        assert!(pc.manifest.symbols.len() > 500);
        assert!(pc.manifest.rules.len() > 100);
        assert!(pc.color_profile.is_some());
        let cp = pc.color_profile.as_ref().unwrap();
        assert!(cp.palette("Day").is_some());
        assert!(cp.palette("Night").is_some());
    }

    #[test]
    fn color_profile_round_trip() {
        let cp = parse_color_profile_xml(MINIMAL_COLOR_PROFILE.as_bytes()).unwrap();
        assert_eq!(cp.tokens.len(), 1);
        assert_eq!(cp.tokens[0].token, "CHRED");
        assert_eq!(cp.palettes.len(), 2);
        let day = cp.palette("Day").unwrap();
        assert_eq!(day.css.as_deref(), Some("daySvgStyle.css"));
        assert_eq!(day.srgb("CHRED"), Some((255, 0, 0)));
        assert_eq!(day.items[0].cie_xy_l, Some((0.55, 0.30, 30.0)));
        assert_eq!(cp.palette("Night").unwrap().srgb("CHRED"), Some((96, 0, 0)));
    }
}
