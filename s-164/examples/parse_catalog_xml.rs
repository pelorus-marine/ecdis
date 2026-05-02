//! Parse a standalone **S-100 exchange catalogue** (`CATALOG.XML`, UTF-8).
//!
//! ```bash
//! unzip -p ~/Downloads/S-64_1.2.0.zip 'S-100/DisplayStandard/S100_ROOT/CATALOG.XML' > /tmp/CATALOG.XML
//! cargo run -p s-164 --example parse_catalog_xml -- /tmp/CATALOG.XML
//! ```

use std::env;
use std::fs;

use s_164::parse_exchange_catalogue;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).ok_or(
        "usage: cargo run -p s-164 --example parse_catalog_xml -- <CATALOG.XML>",
    )?;
    let xml = fs::read(path)?;
    let catalogue = parse_exchange_catalogue(&xml)?;
    println!("catalogue_identifier: {:?}", catalogue.catalogue_identifier);
    println!("datasets: {}", catalogue.datasets.len());
    for ds in &catalogue.datasets {
        println!(
            "  product={} uri={}",
            ds.product_identifier.as_deref().unwrap_or("—"),
            ds.file_uri
        );
    }
    Ok(())
}
