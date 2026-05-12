//! Integration tests parsing real **S-101 ENC** datasets from the **IHO S-164** corpus.
//!
//! Both tests use [`s_164::Corpus`] — `s-164` owns download, caching, SHA-256 verification,
//! and dataset discovery. This test only consumes the bytes it hands back.
//!
//! * [`parses_every_s101_dataset_in_corpus`] (default) opens a local zip from
//!   **`IHO_TESTDATA_ZIP`**. Mirrors the existing CI variable.
//! * [`fetches_and_parses_default_corpus`] (`#[ignore]`, opt-in) calls
//!   [`Corpus::fetch_default`] which downloads + caches the pinned IHO v1.2.0 asset.

use std::io::{BufReader, Cursor};

use iso8211::DataDescriptiveFile;
use s_164::{Corpus, DatasetEntry};

const S101_PRODUCT_ID: &str = "S-101";

#[test]
#[ignore = "requires IHO_TESTDATA_ZIP pointing at the S-164 corpus zip"]
fn parses_every_s101_dataset_in_corpus() {
    let zip_path = std::env::var_os("IHO_TESTDATA_ZIP")
        .expect("set IHO_TESTDATA_ZIP to the S-164 corpus zip path");
    let mut corpus = Corpus::open(&zip_path).unwrap_or_else(|e| panic!("open {:?}: {e}", zip_path));
    assert_s101_corpus_parses(&mut corpus);
}

#[test]
#[ignore = "network: downloads ~6 MB GitHub release asset (cached after first run)"]
fn fetches_and_parses_default_corpus() {
    let mut corpus = Corpus::fetch_default().expect("fetch default IHO S-164 corpus");
    assert_s101_corpus_parses(&mut corpus);
}

fn assert_s101_corpus_parses(corpus: &mut Corpus) {
    let entries: Vec<DatasetEntry> =
        corpus.datasets_for_product(S101_PRODUCT_ID).cloned().collect();
    assert!(!entries.is_empty(), "corpus advertises no S-101 datasets");

    let mut positives_ok = 0usize;
    let mut negatives_ok = 0usize;
    let mut positive_failures: Vec<(String, String)> = Vec::new();
    let mut negative_unexpected_success: Vec<String> = Vec::new();

    for entry in &entries {
        let bytes = corpus
            .read_dataset(entry)
            .unwrap_or_else(|e| panic!("read {}: {e}", entry.zip_path));
        let outcome = DataDescriptiveFile::read_buf(BufReader::new(Cursor::new(bytes)));
        let expect_failure = entry.classification.expects_iso8211_parse_failure();

        match (expect_failure, outcome) {
            (false, Ok(ddf)) => {
                let ddr = ddf.data_descriptive_record();
                assert!(
                    !ddr.data_descriptive_fields().is_empty(),
                    "no DDR data descriptive fields in {}",
                    entry.zip_path
                );
                assert!(
                    !ddf.data_records().is_empty(),
                    "no data records in {}",
                    entry.zip_path
                );
                eprintln!(
                    "OK  {} ({} tag pairs, {} DD fields, {} records)",
                    entry.zip_path,
                    ddr.file_control_field().tag_pairs().len(),
                    ddr.data_descriptive_fields().len(),
                    ddf.data_records().len()
                );
                positives_ok += 1;
            }
            (false, Err(e)) => positive_failures.push((entry.zip_path.clone(), e.to_string())),
            (true, Err(e)) => {
                eprintln!("REJECTED (expected) {}: {e}", entry.zip_path);
                negatives_ok += 1;
            }
            (true, Ok(_)) => negative_unexpected_success.push(entry.zip_path.clone()),
        }
    }

    assert!(positives_ok > 0, "no positive S-101 datasets exercised");
    assert!(
        negatives_ok > 0,
        "no negative-case S-101 datasets exercised; \
         update s-164's Classification mapping for this corpus edition"
    );
    assert!(
        positive_failures.is_empty(),
        "{positives_ok} parsed but {} unexpectedly failed:\n{}",
        positive_failures.len(),
        positive_failures
            .iter()
            .map(|(p, e)| format!("  {p}: {e}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert!(
        negative_unexpected_success.is_empty(),
        "deliberately corrupt datasets parsed without error:\n{}",
        negative_unexpected_success
            .iter()
            .map(|p| format!("  {p}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
