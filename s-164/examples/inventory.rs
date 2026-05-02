//! Inspect an S-164-style zip: exchange sets, catalogue ids, and dataset paths.
//!
//! ## Local file (no network)
//!
//! ```bash
//! cargo run -p s-164 --example inventory -- local /path/to/S-64_1.2.0.zip
//! ```
//!
//! ## Download default corpus (GitHub v1.2.0 `S-64_1.2.0.zip`)
//!
//! ```bash
//! cargo run -p s-164 --example inventory -- download
//! ```
//!
//! ## Download custom URL
//!
//! ```bash
//! cargo run -p s-164 --example inventory -- download 'https://…/bundle.zip'
//! ```

use std::env;
use std::fs;
use std::io::{Read, Seek};

use s_164::{
    discover_exchange_sets, download_bytes, load_exchange_catalogue, resolve_bundle_path,
    DEFAULT_TEST_DATA_ZIP_V1_2_0_URL,
};
use zip::ZipArchive;

enum Mode {
    Local(String),
    Download(String),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mode = parse_mode()?;
    let bytes = match mode {
        Mode::Local(path) => fs::read(&path)?,
        Mode::Download(url) => {
            println!("Fetching:\n  {url}\n");
            let b = download_bytes(&url)?;
            println!("{} bytes\n", b.len());
            b
        }
    };

    let mut archive = ZipArchive::new(std::io::Cursor::new(bytes))?;
    print_inventory(&mut archive)?;
    Ok(())
}

fn parse_mode() -> Result<Mode, &'static str> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("local") => {
            let path = args
                .next()
                .ok_or("usage: cargo run -p s-164 --example inventory -- local <path-to.zip>")?;
            Ok(Mode::Local(path))
        }
        Some("download") => {
            let url = args.next().unwrap_or_else(|| DEFAULT_TEST_DATA_ZIP_V1_2_0_URL.to_string());
            Ok(Mode::Download(url))
        }
        _ => Err("usage:\n\
               cargo run -p s-164 --example inventory -- local <path-to.zip>\n\
               cargo run -p s-164 --example inventory -- download [url]"),
    }
}

fn print_inventory<R: Read + Seek>(
    archive: &mut ZipArchive<R>,
) -> Result<(), Box<dyn std::error::Error>> {
    let sets = discover_exchange_sets(archive)?;
    println!("exchange sets: {}\n", sets.len());

    let show = sets.len().min(25);
    for loc in sets.iter().take(show) {
        let catalogue = load_exchange_catalogue(archive, loc)?;
        println!(
            "{} — catalogue id {:?} — {} datasets",
            loc.prefix.trim_end_matches('/'),
            catalogue.catalogue_identifier,
            catalogue.datasets.len()
        );
        for ds in catalogue.datasets.iter().take(4) {
            let resolved = resolve_bundle_path(&loc.prefix, &ds.file_uri)?;
            let prod = ds.product_identifier.as_deref().unwrap_or("(no productIdentifier)");
            println!("    {prod}: {}", ds.file_uri);
            println!("         → {resolved}");
        }
        if catalogue.datasets.len() > 4 {
            println!("    … {} more datasets", catalogue.datasets.len() - 4);
        }
        println!();
    }

    if sets.len() > show {
        println!(
            "… {} more exchange sets not listed (limit {})",
            sets.len() - show,
            show
        );
    }

    Ok(())
}
