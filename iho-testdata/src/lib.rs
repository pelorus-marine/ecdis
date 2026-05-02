//! Orchestration library: scan an **S-164-style** zip and parse **S-101** datasets ([`run_corpus_zip`]).
//!
//! Used by the **`iho-testdata`** binary and integration tests; keeps [`s_164`] and [`s_101`] independent.

#![forbid(unsafe_code)]

use std::io::{Read, Seek};

use s_101::S101Dataset;
use s_164::{
    discover_exchange_sets, load_exchange_catalogue, read_zip_entry, resolve_bundle_path,
    zip_archive_from_bytes,
};
use zip::ZipArchive;

/// Aggregate result after scanning every exchange set / catalogue row.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CorpusRunSummary {
    pub exchange_sets: usize,
    pub ok: u64,
    pub parse_fail: u64,
    pub io_fail: u64,
    pub skipped_non_s101: u64,
}

impl CorpusRunSummary {
    /// Human-readable line matching the CLI footer (`stderr`).
    #[must_use]
    pub fn summary_line(&self) -> String {
        format!(
            "summary: ok={} parse_fail={} io_fail={} skipped_non_s101_rows={} exchange_sets={}",
            self.ok, self.parse_fail, self.io_fail, self.skipped_non_s101, self.exchange_sets
        )
    }
}

/// Read a corpus zip from bytes and parse each **S-101** catalogue entry.
///
/// When `verbose`, prints one **stdout** line per successful parse and **`eprintln!`** for failures.
pub fn run_corpus_zip(bytes: &[u8], verbose: bool) -> Result<CorpusRunSummary, s_164::S164Error> {
    let mut archive = zip_archive_from_bytes(bytes.to_vec())?;
    run_corpus_archive(&mut archive, verbose)
}

/// Run against an already-open zip archive.
pub fn run_corpus_archive<R: Read + Seek>(
    archive: &mut ZipArchive<R>,
    verbose: bool,
) -> Result<CorpusRunSummary, s_164::S164Error> {
    let sets = discover_exchange_sets(archive)?;

    let mut summary = CorpusRunSummary {
        exchange_sets: sets.len(),
        ..Default::default()
    };

    for loc in &sets {
        let catalogue = load_exchange_catalogue(archive, loc)?;
        for ds in &catalogue.datasets {
            if !is_s101_dataset(ds) {
                summary.skipped_non_s101 += 1;
                continue;
            }

            let zip_path = resolve_bundle_path(&loc.prefix, &ds.file_uri)?;
            let cell = match read_zip_entry(archive, &zip_path) {
                Ok(b) => b,
                Err(e) => {
                    if verbose {
                        eprintln!("SKIP read {zip_path}: {e}");
                    }
                    summary.io_fail += 1;
                    continue;
                }
            };

            match S101Dataset::load_bytes(&cell) {
                Ok(enc) => {
                    if verbose {
                        let dsid_note = if enc.first_record_dsid_payload().is_some() {
                            "has DSID payload"
                        } else {
                            "no DSID payload"
                        };
                        println!(
                            "OK {zip_path} — records={} ({dsid_note}) [{}]",
                            enc.record_count(),
                            catalogue.catalogue_identifier
                        );
                    }
                    summary.ok += 1;
                }
                Err(e) => {
                    if verbose {
                        eprintln!("FAIL parse {zip_path}: {e}");
                    }
                    summary.parse_fail += 1;
                }
            }
        }
    }

    Ok(summary)
}

fn is_s101_dataset(ds: &s_164::DatasetDiscovery) -> bool {
    ds.product_identifier.as_deref() == Some("S-101") || ds.file_uri.contains("/S-101/")
}
