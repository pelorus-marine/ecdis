# iso8211

Rust parser for **ISO/IEC 8211** exchange files—the binary interchange format used by the **International Hydrographic Organization (IHO)** for **S-57** and **S-100-family** products (including **S-101 ENC** datasets on the wire).

[![Documentation](https://docs.rs/iso8211/badge.svg)](https://docs.rs/iso8211)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

## Relation to IHO standards

| IHO / ISO | Role |
|-----------|------|
| **ISO/IEC 8211** | Logical record format (leader, directory, fields). This crate implements **that** layer. |
| **S-57 / S-100 / S-101** | **Product** and **data model** specs on top of 8211. Interpreting feature codes, geometry, and portrayal is **not** done here—use product crates such as [`s-101`](https://crates.io/crates/s-101). |

## Quick start

```toml
[dependencies]
iso8211 = "0.1.5"
```

```rust
use iso8211::DataDescriptiveFile;

fn main() -> Result<(), iso8211::Iso8211Error> {
    let ddf = DataDescriptiveFile::read("dataset.000")?;
    println!("DDR fields: {}", ddf.data_descriptive_record().data_descriptive_fields().len());
    println!("data records: {}", ddf.data_records().len());
    Ok(())
}
```

Example (dump structure of a file):

```bash
cargo run -p iso8211 --example print -- path/to/file.000
```

## What this crate does

- Reads the **Data Descriptive Record** (leader, directory, file control field, data descriptive fields).
- Reads each **Data Record** with **parallel field tags** and raw payloads.
- Parses **format control** strings into a [`Format`](https://docs.rs/iso8211/latest/iso8211/ddr/enum.Format.html) AST.

## Further reading

- Design: [ARCHITECTURE.md](ARCHITECTURE.md) in this crate directory.
- Workspace: [../ARCHITECTURE.md](../ARCHITECTURE.md).

## License

Licensed under **either** of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

**at your option.**

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate, as defined in the Apache-2.0 license, shall be **dual licensed** as above, without additional terms or conditions.

Normative ENC / S-100 behaviour remains defined **only** by **IHO publications** and applicable **flag-State** rules.
