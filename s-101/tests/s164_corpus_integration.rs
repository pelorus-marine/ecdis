//! Integration test: walk an **S-164** corpus via [`s_164::Corpus`], decode every advertised
//! S-101 dataset with [`S101Dataset`], instantiate the geometry / feature-inventory objects
//! the crate exposes, and print a per-dataset + corpus-wide summary.
//!
//! Stack exercised: `s_164` (bytes) → `iso8211` (DDR/DR via `S101Dataset::load_bytes`) → `s_101`.
//!
//! ```bash
//! IHO_TESTDATA_ZIP=/tmp/S-64_1.2.0.zip \
//!   cargo test -p s-101 --test s164_corpus_integration -- --ignored --nocapture
//! ```

use s_101::{
    FeatureInventorySummary, IntegerCrsParameters, S101Dataset, extract_c2il_polylines_wgs84,
};
use s_164::{Corpus, DatasetEntry};

const S101_PRODUCT_ID: &str = "S-101";
const MAX_POLYLINE_POINTS_PER_DATASET: usize = 50_000;

#[test]
#[ignore = "requires IHO_TESTDATA_ZIP pointing at the S-164 corpus zip"]
fn decodes_and_summarises_corpus_from_local_zip() {
    let zip_path = std::env::var_os("IHO_TESTDATA_ZIP")
        .expect("set IHO_TESTDATA_ZIP to the S-164 corpus zip path");
    let mut corpus =
        Corpus::open(&zip_path).unwrap_or_else(|e| panic!("open {:?}: {e}", zip_path));
    summarise_corpus(&mut corpus);
}

#[test]
#[ignore = "network: downloads ~6 MB GitHub release asset (cached after first run)"]
fn decodes_and_summarises_default_corpus() {
    let mut corpus = Corpus::fetch_default().expect("fetch default IHO S-164 corpus");
    summarise_corpus(&mut corpus);
}

#[derive(Debug, Default, Clone, Copy)]
struct CorpusTotals {
    datasets_loaded: usize,
    datasets_rejected_as_expected: usize,
    total_records: usize,
    total_feature_records: usize,
    datasets_with_crs: usize,
    polylines: usize,
    polyline_points: usize,
}

fn summarise_corpus(corpus: &mut Corpus) {
    let entries: Vec<DatasetEntry> = corpus
        .datasets_for_product(S101_PRODUCT_ID)
        .cloned()
        .collect();
    assert!(!entries.is_empty(), "corpus advertises no S-101 datasets");

    eprintln!(
        "\n--- decoding {} S-101 datasets ({} exchange sets) ---\n",
        entries.len(),
        corpus.exchange_sets().len(),
    );

    let mut totals = CorpusTotals::default();
    let mut unexpected_load_failures: Vec<(String, String)> = Vec::new();
    let mut unexpected_load_successes: Vec<String> = Vec::new();

    for entry in &entries {
        let bytes = corpus
            .read_dataset(entry)
            .unwrap_or_else(|e| panic!("read {}: {e}", entry.zip_path));

        match (
            entry.classification.expects_iso8211_parse_failure(),
            S101Dataset::load_bytes(&bytes),
        ) {
            (false, Ok(dataset)) => {
                let row = DatasetReport::from_dataset(&dataset, entry);
                row.print();
                totals.accumulate(&row);
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
         datasets loaded            : {}\n\
         datasets rejected (expected): {}\n\
         total data records         : {}\n\
         total feature records      : {}\n\
         datasets with decodable CRS: {} / {}\n\
         total C2IL polylines       : {}\n\
         total polyline points      : {}\n",
        totals.datasets_loaded,
        totals.datasets_rejected_as_expected,
        totals.total_records,
        totals.total_feature_records,
        totals.datasets_with_crs,
        totals.datasets_loaded,
        totals.polylines,
        totals.polyline_points,
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
    assert!(
        totals.total_feature_records > 0,
        "corpus exposed no FRID-bearing feature records"
    );
    assert!(
        totals.datasets_rejected_as_expected > 0,
        "no NegativeBytes datasets exercised — update s-164 Classification mapping?"
    );
}

struct DatasetReport<'a> {
    entry: &'a DatasetEntry,
    inventory: FeatureInventorySummary,
    crs: Option<IntegerCrsParameters>,
    polyline_count: usize,
    point_count: usize,
    dsid_payload_len: Option<usize>,
}

impl<'a> DatasetReport<'a> {
    fn from_dataset(dataset: &S101Dataset, entry: &'a DatasetEntry) -> Self {
        let inventory = dataset.feature_inventory_summary();
        let crs = dataset.integer_crs_parameters();
        let (_resolved_crs, chains) =
            extract_c2il_polylines_wgs84(dataset, MAX_POLYLINE_POINTS_PER_DATASET);
        let point_count = chains.iter().map(Vec::len).sum();
        let dsid_payload_len = dataset.first_record_dsid_payload().map(<[u8]>::len);

        Self {
            entry,
            inventory,
            crs,
            polyline_count: chains.len(),
            point_count,
            dsid_payload_len,
        }
    }

    fn print(&self) {
        let crs = match self.crs {
            Some(c) => format!(
                "CRS(dco=({:.3},{:.3}) cmf=({},{}))",
                c.dco_x, c.dco_y, c.cmf_x, c.cmf_y
            ),
            None => "CRS(–)".to_string(),
        };
        let dsid = match self.dsid_payload_len {
            Some(n) => format!("DSID={n}B"),
            None => "DSID=–".to_string(),
        };
        eprintln!(
            "OK  {path}  records={records} features={features} polylines={polylines} points={points}  {crs}  {dsid}",
            path = self.entry.zip_path,
            records = self.inventory.total_data_records,
            features = self.inventory.records_with_frid,
            polylines = self.polyline_count,
            points = self.point_count,
        );
    }
}

impl CorpusTotals {
    fn accumulate(&mut self, row: &DatasetReport<'_>) {
        self.datasets_loaded += 1;
        self.total_records += row.inventory.total_data_records;
        self.total_feature_records += row.inventory.records_with_frid;
        if row.crs.is_some() {
            self.datasets_with_crs += 1;
        }
        self.polylines += row.polyline_count;
        self.polyline_points += row.point_count;
    }
}
