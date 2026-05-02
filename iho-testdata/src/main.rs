//! End-to-end: fetch published IHO test corpus (zip) then parse S-101 datasets inside it.
//!
//! Orchestration only: depends on `s_164` and `s_101`; neither library depends on the other.
//!
//! ```bash
//! cargo run -p iho-testdata -- download
//! cargo run -p iho-testdata -- local ./S-64_1.2.0.zip
//! ```

#![forbid(unsafe_code)]

use std::env;

use iho_testdata::{run_corpus_zip, CorpusRunSummary};
use s_164::{download_bytes, DEFAULT_TEST_DATA_ZIP_V1_2_0_URL};

enum Mode {
    Local(String),
    Download(String),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mode = parse_mode()?;
    let bytes = match mode {
        Mode::Local(path) => std::fs::read(path)?,
        Mode::Download(url) => {
            eprintln!("Fetching:\n  {url}\n");
            download_bytes(&url)?
        }
    };

    let summary = run_corpus_zip(&bytes, true)?;
    print_summary(&summary);

    Ok(())
}

fn print_summary(summary: &CorpusRunSummary) {
    eprintln!("{}", summary.summary_line());
}

fn parse_mode() -> Result<Mode, &'static str> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("local") => {
            let path = args.next().ok_or(
                "usage: cargo run -p iho-testdata -- local <path-to-S-164-corpus.zip>",
            )?;
            Ok(Mode::Local(path))
        }
        Some("download") => {
            let url = args
                .next()
                .unwrap_or_else(|| DEFAULT_TEST_DATA_ZIP_V1_2_0_URL.to_string());
            Ok(Mode::Download(url))
        }
        _ => Err(
            "usage:\n\
               cargo run -p iho-testdata -- local <path-to.zip>\n\
               cargo run -p iho-testdata -- download [url]",
        ),
    }
}
