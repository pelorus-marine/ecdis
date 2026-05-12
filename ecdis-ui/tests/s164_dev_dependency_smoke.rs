//! Keeps **`s-164`** as a meaningful **[dev-dependencies]** link: corpus layout discovery only
//! (no ENC semantics — the UI binary loads loose `.000` files).

#![forbid(unsafe_code)]

use std::fs::File;

use s_164::discover_exchange_sets;
use zip::ZipArchive;

#[test]
fn discover_exchange_sets_when_s64_zip_present() {
    let root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../target/iho-cache/S-64_1.2.0.zip");
    if !root.exists() {
        return;
    }
    let file = File::open(root).expect("open S-64 zip");
    let mut archive = ZipArchive::new(file).expect("zip archive");
    let sets = discover_exchange_sets(&mut archive).expect("discover exchange sets");
    assert!(
        sets.iter().any(|s| s.prefix.contains("DisplayBase")),
        "expected DisplayBase exchange set in official S-64 corpus: {sets:?}"
    );
}
