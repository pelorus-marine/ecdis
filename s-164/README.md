# s-164

**IHO S-164** — IHO test data sets for S-100 ECDIS

Support crate for **conformance / test datasets** associated with S-100 ECDIS: download published zip corpora, discover exchange sets, parse `CATALOG.XML` discovery metadata.

This repository targets **[Pelorus](https://sevenseas.io/pelorus) ECDIS / S-100** tooling. The **s-164** crate aligns with hyphenated **S-xxx** naming on [crates.io](https://crates.io/crates/s-164). Dataset bytes can be passed to [`s-101`](https://crates.io/crates/s-101) after resolving paths with `resolve_bundle_path`.

Normative text always comes from **IHO publications** for the edition you certify against.

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

- [`iso8211`](../iso8211/) — ISO/IEC 8211 interchange
- [`s-100`](../s-100/) — S-100 universal data model (framework)
- Workspace index: [../ARCHITECTURE.md](../ARCHITECTURE.md)

## License

Licensed under **MIT OR Apache-2.0**:

- [LICENSE-MIT](LICENSE-MIT)
- [LICENSE-APACHE](LICENSE-APACHE)

at your option.
