//! Integration tests against the published **S-64** corpus (opt-in via **`IHO_TESTDATA_ZIP`**).
//!
//! CI sets **`IHO_TESTDATA_ZIP`** and runs **`cargo test -p iho-testdata s64_corpus_meets_thresholds -- --ignored`** (see `.github/workflows/ci.yml`).
//! Thresholds live in **`expectations/s64_v1_2_0.json`** — adjust when you intentionally upgrade the bundle.

use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Thresholds {
    #[allow(dead_code)]
    schema_version: u32,
    #[allow(dead_code)]
    pinned_release_tag: String,
    #[allow(dead_code)]
    pinned_asset: String,
    min_exchange_sets: u64,
    min_parsed_s101_ok: u64,
    max_parse_fail: u64,
    max_io_fail: u64,
}

fn default_expectations_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("expectations/s64_v1_2_0.json")
}

fn load_thresholds() -> Thresholds {
    let path = std::env::var_os("IHO_EXPECTATIONS_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(default_expectations_path);
    let raw = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_json::from_str(&raw).unwrap_or_else(|e| panic!("parse {}: {e}", path.display()))
}

#[test]
fn expectations_json_matches_schema() {
    let _ = load_thresholds();
}

#[test]
#[ignore = "requires IHO_TESTDATA_ZIP (CI sets it and runs with --ignored)"]
fn s64_corpus_meets_thresholds() {
    let zip_path = std::env::var_os("IHO_TESTDATA_ZIP").expect("IHO_TESTDATA_ZIP must be set");
    let bytes = fs::read(&zip_path).unwrap_or_else(|e| panic!("read {:?}: {e}", zip_path));
    let summary = iho_testdata::run_corpus_zip(&bytes, false).expect("corpus scan");

    let exp = load_thresholds();

    assert!(
        summary.exchange_sets as u64 >= exp.min_exchange_sets,
        "exchange_sets {} < min_exchange_sets {} ({})",
        summary.exchange_sets,
        exp.min_exchange_sets,
        summary.summary_line()
    );
    assert!(
        summary.ok >= exp.min_parsed_s101_ok,
        "ok {} < min_parsed_s101_ok {} ({})",
        summary.ok,
        exp.min_parsed_s101_ok,
        summary.summary_line()
    );
    assert!(
        summary.parse_fail <= exp.max_parse_fail,
        "parse_fail {} > max_parse_fail {} ({})",
        summary.parse_fail,
        exp.max_parse_fail,
        summary.summary_line()
    );
    assert!(
        summary.io_fail <= exp.max_io_fail,
        "io_fail {} > max_io_fail {} ({})",
        summary.io_fail,
        exp.max_io_fail,
        summary.summary_line()
    );
}
