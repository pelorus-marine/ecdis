# s-101

**IHO S-101** — *Electronic Navigational Chart (ENC)* — first Rust **load + validate** slice from **ISO 8211**.

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

## IHO relationship

**S-101** defines the **vector ENC product** in the **S-100** family. It replaces **S-57** ENC for new production in the IHO roadmap. This crate:

- Reads exchange files via [`iso8211`](https://crates.io/crates/iso8211) (**ISO/IEC 8211**).
- Validates an **S-101-shaped** dataset (DDR includes dataset identification; first data record carries **`DSID`** directory tag).
- Exposes **tag → payload** helpers and an **FRID-based feature inventory** (`FeatureInventorySummary`) with a pinned **`TARGET_PRODUCT_SPECIFICATION_EDITION`** label.

Full **feature catalogue XML** binding, geometry decode, attributes, and portrayal are **not** implemented yet.

## Quick start

```toml
[dependencies]
s-101 = "0.0.1"
iso8211 = "0.1.5"   # optional direct access alongside s-101
```

```rust
use s_101::S101Dataset;

fn main() -> Result<(), s_101::S101Error> {
    let enc = S101Dataset::load("path/to/dataset.000")?;
    // Or from bytes (e.g. zip member):
    // let enc = S101Dataset::load_bytes(&buf)?;
    println!("records: {}", enc.record_count());
    Ok(())
}
```

Optional test fixture: place a licensed **S-101** `.000` at **`testdata/s101_sample.000`** (workspace root); see [../../testdata/README.md](../../testdata/README.md).

## Documentation

- [ARCHITECTURE.md](ARCHITECTURE.md)
- [../../ARCHITECTURE.md](../../ARCHITECTURE.md)
- Bridge for sensors + chart: `pelorus-adapter` in this workspace ([pelorus-adapter/README.md](../../pelorus-adapter/README.md))

## License

**MIT OR Apache-2.0** — [LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE).

at your option. IHO text remains © IHO; this crate is an **independent** implementation effort.
