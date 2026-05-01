# s-100

**IHO S-100** — *Universal Hydrographic Data Model* (framework for S-100-based product specifications).

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

## Purpose and IHO relationship

**S-100** is the IHO’s modern **common data framework** (aligned with ISO 191xx concepts) for digital hydrographic and nautical products. Individual **product specifications** (**S-101**, **S-102**, …) build on **S-100**.

This crate is a **placeholder (v0.0.1)** to reserve the hyphenated name **`s-100`** on [crates.io](https://crates.io/crates/s-100) and to grow **shared Rust types** (metadata, common identifiers, catalogue hooks) so product crates do not duplicate framework-level concepts.

Transport encoding of datasets typically uses **ISO 8211** — see the [`iso8211`](https://crates.io/crates/iso8211) crate.

## Status

**Stub:** no schema binding yet. See [ARCHITECTURE.md](ARCHITECTURE.md).

## License

Licensed under **MIT OR Apache-2.0**:

- [LICENSE-MIT](LICENSE-MIT)
- [LICENSE-APACHE](LICENSE-APACHE)

at your option.

Normative definitions are **only** in IHO **S-100** and related publications for the edition you target.
