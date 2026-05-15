# s-164

**IHO S-164** — IHO test data sets for S-100 ECDIS

Abstraction layer over **conformance / test datasets** for S-100 ECDIS: download and SHA-256-verify the published zip corpus, cache it on disk, expose a typed index of exchange sets and datasets, and hand back raw bytes ready for product decoders such as [`s-101`](https://crates.io/crates/s-101). Lower-level primitives (`discover_exchange_sets`, `read_zip_entry`, `resolve_bundle_path`, …) remain available for callers that need them.

This repository targets **[Pelorus](https://sevenseas.io/pelorus) ECDIS / S-100** tooling. The **s-164** crate aligns with hyphenated **S-xxx** naming on [crates.io](https://crates.io/crates/s-164).

Normative text always comes from **IHO publications** for the edition you certify against.

## Front-door API

```rust
use s_164::Corpus;

let mut corpus = Corpus::fetch_default()?;          // download + cache + verify
// or: Corpus::open("./S-64_1.2.0.zip")?;           // local zip
// or: Corpus::from_bytes(zip_bytes)?;              // in-memory

for dataset in corpus.datasets_for_product("S-101").cloned().collect::<Vec<_>>() {
    let bytes = corpus.read_dataset(&dataset)?;
    // hand `bytes` to your product decoder
}
# Ok::<(), s_164::S164Error>(())
```

- Downloaded archives are cached under `dirs::cache_dir()/pelorus-marine/s-164/` (override with **`S164_CACHE_DIR`**).
- `fetch_default()` verifies the cached / downloaded bytes against
  [`DEFAULT_TEST_DATA_ZIP_V1_2_0_SHA256`].
- Each [`DatasetEntry`] carries a [`Classification`] derived from its exchange-set prefix
  (`Positive`, `NegativeBytes` for `CorruptData/`, `NegativeUpdateSequence` for `InvalidSequence*/`).

## Examples

From the workspace root:

```bash
# Inventory a zip already on disk (no network)
cargo run -p s-164 --example inventory -- local ./S-64_1.2.0.zip

# Download the default GitHub v1.2.0 bundle and inventory it
cargo run -p s-164 --example inventory -- download

# Parse one extracted CATALOG.XML
cargo run -p s-164 --example parse_catalog_xml -- ./CATALOG.XML
```

See source: [`examples/inventory.rs`](examples/inventory.rs), [`examples/parse_catalog_xml.rs`](examples/parse_catalog_xml.rs).

## Related workspace crates

- [`iso8211`](../../iso8211/) — ISO/IEC 8211 interchange
- [`s-100`](../s-100/) — S-100 universal data model (framework)
- Workspace index: [../ARCHITECTURE.md](../../ARCHITECTURE.md)

## License

Licensed under **MIT OR Apache-2.0**:

- [LICENSE-MIT](LICENSE-MIT)
- [LICENSE-APACHE](LICENSE-APACHE)

at your option.
