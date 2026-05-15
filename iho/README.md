# IHO product crates (`iho/`)

Rust libraries for **IHO S-xxx** specifications in the **S-100** family (and related products). Crate names stay hyphenated **`s-*`** on [crates.io](https://crates.io/); directories live under **`iho/`** in this workspace.

| Crate | Standard | Notes |
|-------|----------|--------|
| [s-100](s-100/) | S-100 | Shared geometry / identifiers (stub). |
| [s-101](s-101/) | S-101 | ENC — ISO 8211 load, feature graph (active). |
| [s-164](s-164/) | S-164 | Test corpus zip discovery (no product decode). |
| [s-61](s-61/) … [s-131](s-131/) | Various | Placeholders / stubs — see each `README.md`. |

**Transport:** [iso8211](../iso8211/) (ISO/IEC 8211). **Integration:** [pelorus-adapter](../pelorus-adapter/) (chart + Core navigation snapshots).

Full workspace index: [../README.md#crate-index](../README.md#crate-index).
