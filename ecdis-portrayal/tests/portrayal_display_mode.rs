//! Display mode + catalogue bundle integration (optional IHO zip).

use ecdis_portrayal::{
    ChartTheme, ChartViewport, CpuOutlinePortrayal, DisplayMode, PortrayalInputs, ViewerScene,
    build_frame, build_theme_swatches_frame,
};
use s_101::{PortrayalCatalogueBundle, S101Dataset};

#[test]
fn theme_fallback_differs_by_mode() {
    let day = ChartTheme::resolve(DisplayMode::Day, None);
    let night = ChartTheme::resolve(DisplayMode::Night, None);
    assert_ne!(day.background, night.background);
}

#[test]
#[ignore = "requires IHO_TESTDATA_ZIP pointing at the S-64 corpus zip"]
fn catalogue_palettes_and_stylesheets() {
    let zip_path = std::env::var_os("IHO_TESTDATA_ZIP")
        .expect("set IHO_TESTDATA_ZIP to the S-64 corpus zip path");
    let mut corpus = s_164::Corpus::open(&zip_path).expect("open corpus");
    let entry = corpus
        .portrayal_catalogues()
        .find(|c| c.product_identifier.as_deref() == Some("S-101"))
        .cloned()
        .expect("S-101 portrayal catalogue");
    let bytes = corpus.read_catalogue(&entry).expect("read catalogue");
    let bundle = PortrayalCatalogueBundle::open_zip(std::sync::Arc::<[u8]>::from(bytes))
        .expect("open bundle");

    let names = bundle.palette_names();
    assert!(names.iter().any(|n| n.eq_ignore_ascii_case("Day")));
    assert!(names.iter().any(|n| n.eq_ignore_ascii_case("Night")));

    let day = bundle.catalogue.palette("Day").expect("Day palette");
    assert!(day.css.is_some());
    let css = bundle.read_palette_stylesheet(day).expect("day stylesheet");
    assert!(!css.is_empty());
}

#[cfg(feature = "symbols")]
#[test]
#[ignore = "requires IHO_TESTDATA_ZIP; cargo test -p ecdis-portrayal --features symbols,s64"]
fn symbol_day_vs_night_rgba_differs() {
    use ecdis_portrayal::open_s101_portrayal_from_s64_zip;
    use ecdis_portrayal::rasterize_symbol;

    let zip_path = std::env::var_os("IHO_TESTDATA_ZIP").expect("IHO_TESTDATA_ZIP");
    let bundle = open_s101_portrayal_from_s64_zip(&zip_path).expect("open portrayal from S-64");
    let symbol_id = bundle
        .catalogue
        .manifest
        .symbols
        .first()
        .map(|s| s.id.as_str())
        .expect("symbol");
    let day = rasterize_symbol(&bundle, symbol_id, DisplayMode::Day, 96).expect("day raster");
    let night = rasterize_symbol(&bundle, symbol_id, DisplayMode::Night, 96).expect("night raster");
    assert_eq!(day.width_px, night.width_px);
    assert_ne!(day.rgba, night.rgba);
}

#[test]
fn frame_builders_smoke_without_catalogue() {
    let enc = std::env::var("ECDIS_TEST_ENC").ok();
    let Some(enc_path) = enc else {
        eprintln!("skip frame_builders_smoke_without_catalogue: set ECDIS_TEST_ENC");
        return;
    };
    let chart = S101Dataset::load(&enc_path).expect("enc");
    let mut viewport = ChartViewport::new(CpuOutlinePortrayal::default());
    viewport.reset_chart(&chart).expect("reset");
    let inputs = PortrayalInputs {
        chart: &chart,
        feature_catalogue: None,
        catalogue: None,
        viewport: &viewport.state,
        display_mode: DisplayMode::Day,
        outline: viewport.portrayal_ref(),
        selected_symbol_id: None,
    };
    let frame = build_frame(&inputs, ViewerScene::Chart);
    assert!(frame.width_px > 0.0);
    let swatches = build_theme_swatches_frame(&inputs);
    assert!(!swatches.layers.swatches.is_empty());
}
