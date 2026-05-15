//! Integration test: walk an **S-164** corpus via [`s_164::Corpus`], decode every **positive**
//! **S-101** dataset with [`S101Dataset`], parse the feature catalogue, and build a
//! [`s_101::FeatureGraph`] (typed records + FC-backed attributes + WGS84 geometry). Some IHO
//! exchange features carry **no `ATTR` tuples** (geometry + class only); corpus assertions require
//! FC typing only when dataset-local attributes exist.
//!
//! Stack: `s_164` (bytes) → `iso8211` (via [`S101Dataset::load_bytes`]) → `s_101::record` →
//! `s_101::fc::FeatureCatalogue` → [`S101Dataset::build_feature_graph`].
//!
//! ```bash
//! IHO_TESTDATA_ZIP=/tmp/S-64_1.2.0.zip \
//!   cargo test -p s-101 --test s164_corpus_integration decodes_and_summarises_corpus_from_local_zip -- --ignored --nocapture
//! ```
//!
//! [`Corpus::fetch_default`] stays `#[ignore]` because it downloads the corpus; CI uses a cached
//! zip and **`IHO_TESTDATA_ZIP`** with **`decodes_and_summarises_corpus_from_local_zip`** (see `.github/workflows/ci.yml`).

use std::collections::HashMap;

use s_101::{AttributeValue, FeatureCatalogue, Geometry, S101Dataset};
use s_164::{Classification, Corpus, DatasetEntry};

const S101_PRODUCT_ID: &str = "S-101";

#[test]
#[ignore = "requires IHO_TESTDATA_ZIP pointing at the S-164 corpus zip"]
fn decodes_and_summarises_corpus_from_local_zip() {
    let zip_path = std::env::var_os("IHO_TESTDATA_ZIP")
        .expect("set IHO_TESTDATA_ZIP to the S-164 corpus zip path");
    let mut corpus = Corpus::open(&zip_path).unwrap_or_else(|e| panic!("open {:?}: {e}", zip_path));
    run_corpus_assertions(&mut corpus);
}

#[test]
#[ignore = "network: downloads ~6 MB GitHub release asset (cached after first run)"]
fn decodes_and_summarises_default_corpus() {
    let mut corpus = Corpus::fetch_default().expect("fetch default IHO S-164 corpus");
    run_corpus_assertions(&mut corpus);
}

#[derive(Debug, Default, Clone, Copy)]
struct CorpusTotals {
    datasets_positive: usize,
    datasets_loaded: usize,
    datasets_rejected_as_expected: usize,
    features: usize,
    features_with_dataset_attrs: usize,
    features_with_typed_fc_attrs: usize,
    spatial_unresolved: usize,
    point_geoms: usize,
    curve_geoms: usize,
    surface_geoms: usize,
    multipoint_geoms: usize,
}

fn run_corpus_assertions(corpus: &mut Corpus) {
    let entries: Vec<DatasetEntry> =
        corpus.datasets_for_product(S101_PRODUCT_ID).cloned().collect();
    assert!(!entries.is_empty(), "corpus advertises no S-101 datasets");

    let fallback_fc = find_initial_catalogues_fc_xml(corpus);
    assert!(
        fallback_fc.is_some(),
        "corpus must expose S-101 FC XML under InitialCatalogues for graph tests"
    );
    let fallback_fc = fallback_fc.expect("fc bytes");
    let fallback_fc = std::sync::Arc::new(
        FeatureCatalogue::parse_xml(&fallback_fc)
            .expect("parse fallback S-101 feature catalogue XML"),
    );

    eprintln!(
        "\n--- feature graph over {} S-101 datasets ({} exchange sets) ---\n",
        entries.len(),
        corpus.exchange_sets().len(),
    );

    let mut totals = CorpusTotals::default();
    let mut unexpected_load_failures: Vec<(String, String)> = Vec::new();
    let mut unexpected_load_successes: Vec<String> = Vec::new();
    let mut fc_cache: HashMap<String, std::sync::Arc<FeatureCatalogue>> = HashMap::new();

    for entry in &entries {
        totals.datasets_positive += usize::from(entry.classification == Classification::Positive);
        let bytes = corpus
            .read_dataset(entry)
            .unwrap_or_else(|e| panic!("read {}: {e}", entry.zip_path));

        match (
            entry.classification.expects_iso8211_parse_failure(),
            S101Dataset::load_bytes(&bytes),
        ) {
            (false, Ok(dataset)) => {
                let fc = fc_for_dataset(corpus, entry, &dataset, &fallback_fc, &mut fc_cache);
                let graph = dataset
                    .build_feature_graph(fc.as_ref())
                    .unwrap_or_else(|e| panic!("graph {}: {e}", entry.zip_path));

                assert!(
                    !graph.features.is_empty(),
                    "no features in {}",
                    entry.zip_path
                );
                let mut ds_point = 0usize;
                let mut ds_curve = 0usize;
                let mut ds_surface = 0usize;
                let mut ds_multi = 0usize;
                for feat in &graph.features {
                    if feat.spatial_assoc_count > 0 && feat.geometry.is_empty() {
                        totals.spatial_unresolved += 1;
                    }
                    assert!(
                        feat.class.is_some(),
                        "missing FC class for {:?} {}",
                        feat.foid,
                        entry.zip_path
                    );
                    if feat.attr_source_count > 0 {
                        totals.features_with_dataset_attrs += 1;
                        assert!(
                            !feat.attributes.is_empty() || !feat.skipped_attr_tuples.is_empty(),
                            "ATTR tuples present but neither resolved nor skipped for {:?} {}",
                            feat.foid,
                            entry.zip_path
                        );
                        if !feat.attributes.is_empty()
                            && feat
                                .attributes
                                .iter()
                                .any(|a| !matches!(a.value, AttributeValue::Raw(_)))
                        {
                            totals.features_with_typed_fc_attrs += 1;
                        }
                    }
                    if !feat.geometry.is_empty() {
                        match &feat.geometry {
                            Geometry::Point(_) => {
                                ds_point += 1;
                                totals.point_geoms += 1;
                            }
                            Geometry::MultiPoint(_) => {
                                ds_multi += 1;
                                totals.multipoint_geoms += 1;
                            }
                            Geometry::Curve(_) => {
                                ds_curve += 1;
                                totals.curve_geoms += 1;
                            }
                            Geometry::Surface(_) => {
                                ds_surface += 1;
                                totals.surface_geoms += 1;
                            }
                        }
                    }
                }
                totals.features += graph.features.len();
                totals.datasets_loaded += 1;
                eprintln!(
                    "OK  {}  features={} (point={} curve={} surface={} multipoint={})",
                    entry.zip_path,
                    graph.features.len(),
                    ds_point,
                    ds_curve,
                    ds_surface,
                    ds_multi
                );
            }
            (false, Err(e)) => {
                unexpected_load_failures.push((entry.zip_path.clone(), e.to_string()));
            }
            (true, Err(e)) => {
                eprintln!(
                    "REJECTED (expected, {:?}) {}: {e}",
                    entry.classification, entry.zip_path
                );
                totals.datasets_rejected_as_expected += 1;
            }
            (true, Ok(_)) => unexpected_load_successes.push(entry.zip_path.clone()),
        }
    }

    eprintln!(
        "\n--- corpus totals ---\n\
         positive datasets scanned     : {}\n\
         datasets loaded (graph built) : {}\n\
         datasets rejected (expected)  : {}\n\
         total features                : {}\n\
         features SPAS but empty geom  : {}\n\
         geometry: point / curve / surface / multipoint\n\
         (last row cumulative per-dataset log above may overcount; see assertions)\n",
        totals.datasets_positive,
        totals.datasets_loaded,
        totals.datasets_rejected_as_expected,
        totals.features,
        totals.spatial_unresolved,
    );

    assert!(
        unexpected_load_failures.is_empty(),
        "{} positive S-101 datasets failed to load:\n{}",
        unexpected_load_failures.len(),
        unexpected_load_failures
            .iter()
            .map(|(p, e)| format!("  {p}: {e}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert!(
        unexpected_load_successes.is_empty(),
        "deliberately corrupt datasets loaded without error:\n{}",
        unexpected_load_successes
            .iter()
            .map(|p| format!("  {p}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert!(totals.datasets_loaded > 0, "no positive datasets loaded");
    assert!(totals.features > 0, "no features decoded from corpus");
    assert!(
        totals.features_with_dataset_attrs > 0,
        "no features with ATTR tuples in corpus — graph attribute path untested"
    );
    assert!(
        totals.features_with_typed_fc_attrs > 0,
        "no FC-typed attribute resolutions — catalogue binding likely broken"
    );
    assert!(
        totals.spatial_unresolved * 20 < totals.features.max(1),
        "too many SPAS rows with unresolved geometry (likely update-cell external refs): {}",
        totals.spatial_unresolved
    );
    assert!(totals.point_geoms > 0, "expected point geometries");
    assert!(totals.curve_geoms > 0, "expected curve geometries");
    assert!(totals.surface_geoms > 0, "expected surface geometries");
    assert!(
        totals.datasets_rejected_as_expected > 0,
        "no NegativeBytes datasets exercised — update s-164 Classification mapping?"
    );
}

fn find_initial_catalogues_fc_xml(corpus: &mut Corpus) -> Option<Vec<u8>> {
    let cats: Vec<_> = corpus.catalogues().to_vec();
    for cat in &cats {
        if !cat.zip_path.contains("InitialCatalogues") {
            continue;
        }
        if !cat.product_identifier.as_deref().is_some_and(|p| p.contains("S-101")) {
            continue;
        }
        if cat.looks_like_portrayal_catalogue() {
            continue;
        }
        if cat.compressed == Some(true) {
            continue;
        }
        if !cat.zip_path.to_ascii_lowercase().ends_with(".xml") {
            continue;
        }
        return corpus.read_catalogue(cat).ok();
    }
    None
}

fn fc_for_dataset(
    corpus: &mut Corpus,
    entry: &DatasetEntry,
    dataset: &S101Dataset,
    fallback: &std::sync::Arc<FeatureCatalogue>,
    cache: &mut HashMap<String, std::sync::Arc<FeatureCatalogue>>,
) -> std::sync::Arc<FeatureCatalogue> {
    fn consider(
        dataset: &S101Dataset,
        fc: std::sync::Arc<FeatureCatalogue>,
        best: &mut Option<(std::sync::Arc<FeatureCatalogue>, usize)>,
    ) {
        let Ok(g) = dataset.build_feature_graph(fc.as_ref()) else {
            return;
        };
        let score = g.features.iter().filter(|f| f.class.is_some()).count();
        if best.as_ref().is_none_or(|(_, s)| score > *s) {
            *best = Some((fc, score));
        }
    }

    let cats: Vec<_> = corpus.catalogues().to_vec();
    let mut candidates: Vec<(String, Vec<u8>)> = Vec::new();
    for cat in &cats {
        if cat.exchange_set_index != entry.exchange_set_index {
            continue;
        }
        if !cat.product_identifier.as_deref().is_some_and(|p| p.contains("S-101")) {
            continue;
        }
        if cat.looks_like_portrayal_catalogue() {
            continue;
        }
        if cat.compressed == Some(true) {
            continue;
        }
        if !cat.zip_path.to_ascii_lowercase().ends_with(".xml") {
            continue;
        }
        if let Ok(bytes) = corpus.read_catalogue(cat) {
            candidates.push((cat.zip_path.clone(), bytes));
        }
    }

    let mut best: Option<(std::sync::Arc<FeatureCatalogue>, usize)> = None;
    let mut tried_paths: std::collections::HashSet<String> =
        candidates.iter().map(|(k, _)| k.clone()).collect();
    for (key, bytes) in candidates {
        let fc = if let Some(hit) = cache.get(&key) {
            hit.clone()
        } else if let Ok(fc) = FeatureCatalogue::parse_xml(&bytes) {
            let arc = std::sync::Arc::new(fc);
            cache.insert(key, arc.clone());
            arc
        } else {
            continue;
        };
        consider(dataset, fc, &mut best);
    }
    consider(dataset, fallback.clone(), &mut best);

    let n_feat = dataset.feature_record_count();
    if best.as_ref().map(|(_, s)| *s).unwrap_or(0) < n_feat {
        for cat in &cats {
            if tried_paths.contains(&cat.zip_path) {
                continue;
            }
            if !cat.product_identifier.as_deref().is_some_and(|p| p.contains("S-101")) {
                continue;
            }
            if cat.looks_like_portrayal_catalogue() {
                continue;
            }
            if cat.compressed == Some(true) {
                continue;
            }
            if !cat.zip_path.to_ascii_lowercase().ends_with(".xml") {
                continue;
            }
            tried_paths.insert(cat.zip_path.clone());
            let key = cat.zip_path.clone();
            let fc = if let Some(hit) = cache.get(&key) {
                hit.clone()
            } else if let Ok(bytes) = corpus.read_catalogue(cat)
                && let Ok(fc) = FeatureCatalogue::parse_xml(&bytes)
            {
                let arc = std::sync::Arc::new(fc);
                cache.insert(key, arc.clone());
                arc
            } else {
                continue;
            };
            consider(dataset, fc, &mut best);
            if best.as_ref().is_some_and(|(_, s)| *s >= n_feat) {
                break;
            }
        }
    }

    best.map(|(fc, _)| fc).unwrap_or_else(|| fallback.clone())
}
