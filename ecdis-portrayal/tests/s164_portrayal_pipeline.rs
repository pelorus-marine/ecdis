//! Stage 1 integration: walk a real IHO **S-164** corpus, load the **portrayal catalogue**(s)
//! via [`s_101::PortrayalCatalogue`], wire one into [`CatalogueBackedPortrayal`], and confirm
//! it drafts one entry per `FRID`-bearing feature record for every positive S-101 cell.
//!
//! Lua rule execution is **not** part of stage 1 — drafts reference the top-level rule entry
//! only. Stage 2 will replace that with per-feature symbology produced by the Lua scripts.
//!
//! ```bash
//! IHO_TESTDATA_ZIP=/tmp/S-64_1.2.0.zip \
//!   cargo test -p ecdis-portrayal --test s164_portrayal_pipeline -- --ignored --nocapture
//! ```

use ecdis_portrayal::{CatalogueBackedPortrayal, PortrayalPipeline};
use s_101::{PortrayalCatalogue, S101Dataset};
use s_164::{CatalogueEntry, Corpus, DatasetEntry};

const S101_PRODUCT_ID: &str = "S-101";

#[test]
#[ignore = "requires IHO_TESTDATA_ZIP pointing at the S-164 corpus zip"]
fn drives_pipeline_with_real_portrayal_catalogue() {
    let zip_path = std::env::var_os("IHO_TESTDATA_ZIP")
        .expect("set IHO_TESTDATA_ZIP to the S-164 corpus zip path");
    let mut corpus = Corpus::open(&zip_path).unwrap_or_else(|e| panic!("open {zip_path:?}: {e}"));
    drive_pipeline(&mut corpus);
}

#[test]
#[ignore = "network: downloads ~6 MB GitHub release asset (cached after first run)"]
fn drives_pipeline_with_default_corpus() {
    let mut corpus = Corpus::fetch_default().expect("fetch default IHO S-164 corpus");
    drive_pipeline(&mut corpus);
}

fn drive_pipeline(corpus: &mut Corpus) {
    let pc_entries: Vec<CatalogueEntry> = corpus.portrayal_catalogues().cloned().collect();
    assert!(
        !pc_entries.is_empty(),
        "corpus advertises no portrayal-catalogue rows"
    );

    eprintln!(
        "\n--- portrayal-catalogue rows discovered: {} ---\n",
        pc_entries.len()
    );
    for entry in &pc_entries {
        eprintln!(
            "  {}  (product={}, scope={}, compressed={:?})",
            entry.zip_path,
            entry.product_identifier.as_deref().unwrap_or("?"),
            entry.scope.as_deref().unwrap_or("?"),
            entry.compressed,
        );
    }

    // Load every portrayal catalogue and print its manifest summary.
    let mut loaded: Vec<(CatalogueEntry, PortrayalCatalogue)> = Vec::new();
    for entry in pc_entries {
        let bytes = corpus
            .read_catalogue(&entry)
            .unwrap_or_else(|e| panic!("read {}: {e}", entry.zip_path));
        let catalogue = PortrayalCatalogue::open_zip(&bytes)
            .unwrap_or_else(|e| panic!("open {}: {e}", entry.zip_path));
        print_catalogue_summary(&entry, &catalogue);
        loaded.push((entry, catalogue));
    }
    assert!(loaded.iter().any(|(_, c)| c.manifest.product_id == "S-101"));
    assert!(loaded.iter().all(|(_, c)| !c.manifest.symbols.is_empty()));
    assert!(loaded.iter().all(|(_, c)| !c.manifest.rules.is_empty()));

    // Use the first loaded catalogue as the pipeline backing.
    let pipeline_catalogue = loaded[0].1.clone();
    let total_rules = pipeline_catalogue.manifest.rules.len();
    let subtemplate_rules =
        pipeline_catalogue.manifest.rules.iter().filter(|r| r.is_sub_template()).count();
    let mut pipeline = CatalogueBackedPortrayal::new(pipeline_catalogue)
        .expect("catalogue should have a TopLevelTemplate rule and a Day palette");
    pipeline.set_display_scale(50_000).unwrap();

    eprintln!(
        "\n--- driving pipeline (palette={}, total_rules={}, sub_template_rules={}) over corpus ---\n",
        pipeline.palette_name(),
        total_rules,
        subtemplate_rules,
    );

    let datasets: Vec<DatasetEntry> = corpus
        .datasets_for_product(S101_PRODUCT_ID)
        .filter(|d| !d.classification.expects_iso8211_parse_failure())
        .cloned()
        .collect();
    assert!(!datasets.is_empty(), "no positive S-101 datasets in corpus");

    let mut total_features_drafted = 0usize;
    for entry in &datasets {
        let bytes = corpus
            .read_dataset(entry)
            .unwrap_or_else(|e| panic!("read {}: {e}", entry.zip_path));
        let dataset = match S101Dataset::load_bytes(&bytes) {
            Ok(d) => d,
            Err(e) => panic!("load {}: {e}", entry.zip_path),
        };

        pipeline
            .reset_for_chart(&dataset)
            .expect("pipeline reset must succeed for positive ENC");

        let drafted = pipeline.drafts().len();
        let frid_records = dataset.feature_record_count();
        assert_eq!(
            drafted, frid_records,
            "pipeline must draft one entry per FRID-bearing record"
        );
        total_features_drafted += drafted;

        eprintln!(
            "OK  {}  features={drafted}  (drafted via rule_id={:?})",
            entry.zip_path,
            pipeline.drafts().first().map(|d| d.rule_id.as_str()),
        );
    }

    eprintln!(
        "\n--- stage 1 totals: {} datasets, {} feature drafts via portrayal pipeline ---\n",
        datasets.len(),
        total_features_drafted,
    );

    assert!(total_features_drafted > 0);
}

fn print_catalogue_summary(entry: &CatalogueEntry, catalogue: &PortrayalCatalogue) {
    let m = &catalogue.manifest;
    let palettes: Vec<&str> = catalogue
        .color_profile
        .as_ref()
        .map(|cp| cp.palettes.iter().map(|p| p.name.as_str()).collect())
        .unwrap_or_default();
    let color_tokens = catalogue.color_profile.as_ref().map(|cp| cp.tokens.len()).unwrap_or(0);
    let top_level = m.rules.iter().find(|r| r.is_top_level()).map(|r| r.id.as_str()).unwrap_or("?");
    eprintln!(
        "  [bundle={}] product={} v{} symbols={} line_styles={} area_fills={} rules={} (top={}) color_tokens={} palettes={:?}  ← {}",
        catalogue.bundle_root,
        m.product_id,
        m.version,
        m.symbols.len(),
        m.line_styles.len(),
        m.area_fills.len(),
        m.rules.len(),
        top_level,
        color_tokens,
        palettes,
        entry.zip_path,
    );
}
