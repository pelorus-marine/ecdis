# s-100

**IHO S-100** — *Universal Hydrographic Data Model* (framework for S-100-based product specifications).

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

## Purpose and IHO relationship

**S-100** is the IHO’s modern **common data framework** (aligned with ISO 191xx concepts) for digital hydrographic and nautical products. Individual **product specifications** (**S-101**, **S-102**, …) build on **S-100**.

This crate provides **shared Rust types** used across product crates:

- **Geometry** in WGS84 degrees: `Point2D`, `MultiPoint2D`, `Curve2D`, `Surface2D`, `Geometry`.
- **Identifiers:** `FeatureObjectId` (FOID triple).

See [ARCHITECTURE.md](ARCHITECTURE.md) for boundaries and module layout.

## Status

**Incremental:** geometry + FOID are implemented; broader UHDM modelling remains optional. The `FrameworkStub` type in `src/lib.rs` is retained for early workspace wiring.

## License

Licensed under **MIT OR Apache-2.0**:

- [LICENSE-MIT](LICENSE-MIT)
- [LICENSE-APACHE](LICENSE-APACHE)

at your option.

Normative definitions are **only** in IHO **S-100** and related publications for the edition you target.
