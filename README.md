# ecdis

Pelorus **ECDIS** / **S-100** Rust workspace: **ISO 8211** interchange (`iso8211`) and **IHO S-100 family** product libraries (`s-100`, `s-101`, …).

See [ARCHITECTURE.md](ARCHITECTURE.md) for the workspace index (links to each crate’s `ARCHITECTURE.md` — **no duplicated design detail** at the root). Work backlog: [GitHub Issues](https://github.com/pelorus-marine/ecdis/issues).

**Recent progress:** `s-101` loads ISO 8211 and validates DSID shape; `pelorus-adapter` bundles `S101Dataset` with `OwnShip` / AIS snapshots and Core mapper traits; `iso8211` `DataRecord` exposes `field_tags` aligned with fields; legacy non-hyphen crates.io tombstones live in sibling [`pelorus-legacy-stubs`](https://github.com/pelorus-marine/pelorus-legacy-stubs).

**Ecosystem:** This repo is a chart-grade component in the [Pelorus](https://sevenseas.io/pelorus) program; the high-level system picture lives in the [Pelorus architecture record](https://github.com/pelorus-marine/specifications/blob/main/ARCHITECTURE.md).

## Repository layout

```text
ecdis/
  Cargo.toml
  ARCHITECTURE.md
  scripts/           # e.g. create_backlog_issues.py
  testdata/          # optional s101_sample.000 — see testdata/README.md
  iso8211/
  pelorus-adapter/   # S-101 + own-ship / AIS + Core mapper traits (no CAN stack in-crate)
  ecdis-ui/                  # Slint Wayland ENC HUD — see ecdis-ui/README.md
  ecdis-portrayal-viewer/    # Dev-only portrayal gallery (Slint)
  yocto/             # Companion BitBake stubs — see yocto/meta-pelorus-ecdis/README.md
  iho/               # IHO S-* product crates (s-101 = ENC, s-164 = test corpora, …)
```

Crate index: [below](#crate-index). Design layering: [ARCHITECTURE.md](ARCHITECTURE.md).

## Prerequisites

- Rust **stable** **≥ 1.90** (see `rust-toolchain.toml`: `rustfmt`, `clippy`): matches **`ecdis-ui`** / Slint, **`Rust 2024`** edition (`edition = "2024"` in every crate manifest), and **`pelorus-adapter`** pulling **`pelorus-core`** from **`pelorus-marine/platform`**.
- Linux dev libraries for Slint (`fontconfig`, Wayland, EGL/Mesa, …) — [`ecdis-ui/README.md`](ecdis-ui/README.md).

## Common commands

```bash
# Format (workspace only; avoids formatting sibling `pelorus-platform` path deps)
cargo fmt

# Test entire workspace
cargo test --workspace --verbose

# Lint
cargo clippy --workspace --all-targets -- -D warnings

# Docs (warnings as errors)
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --open

# Example: ISO 8211 structure dump
cargo run -p iso8211 --example print -- path/to/file.000

# Example: Slint ENC HUD (Wayland/X11 session + dev libs — see ecdis-ui/README.md)
cargo run -p ecdis-ui --release -- path/to/cell.000

# Dev-only portrayal gallery (Slint — see ecdis-portrayal-viewer/README.md)
cargo run -p ecdis-portrayal-viewer -- path/to/cell.000
```

## Crate index

Workspace members from [`Cargo.toml`](Cargo.toml). Each row links the crate directory; open **`README.md`** there for usage and **`ARCHITECTURE.md`** for design notes (where present).

### Interchange and foundation

| Crate | Description |
|-------|-------------|
| [iso8211](iso8211/) | ISO 8211 data format parsing (IHO S-57 / S-100 family exchange format). |
| [iho](iho/) | IHO **S-*** product libraries (`s-100`, `s-101`, `s-164`, …) — see [iho/README.md](iho/README.md). |

### ECDIS application

| Crate | Description |
|-------|-------------|
| [ecdis-behaviours](ecdis-behaviours/) | ECDIS behaviour stubs (overscale, alarms) — IMO logic incremental. |
| [ecdis-portrayal](ecdis-portrayal/) | Portrayal library (display modes, frames; no UI). |
| [ecdis-portrayal-viewer](ecdis-portrayal-viewer/) | **Dev-only** Slint gallery for portrayal debugging. |
| [ecdis-runtime](ecdis-runtime/) | Composition-root demo: ENC load + `ChartNavContext` + portrayal/behaviour stubs. |
| [ecdis-ui](ecdis-ui/) | Slint Wayland UI shell for ENC load + `ChartNavContext` (IVI-style demo). |

### Pelorus integration

| Crate | Description |
|-------|-------------|
| [pelorus-adapter](pelorus-adapter/) | S-101 ENC + own-ship / AIS snapshots and Core/Stream mapper traits (no transports). |

### IHO product libraries (`s-*`)

| Crate | Description |
|-------|-------------|
| [s-61](iho/s-61/) | **S-61** raster navigational charts (RNC) — placeholder. |
| [s-97](iho/s-97/) | **S-97** guidelines for S-100 product specifications — placeholder. |
| [s-98](iho/s-98/) | **S-98** data product interoperability — placeholder. |
| [s-99](iho/s-99/) | **S-99** GI registry operational procedures — placeholder. |
| [s-101](iho/s-101/) | **S-101** ENC — decode from ISO 8211 (initial slice). |
| [s-102](iho/s-102/) | **S-102** bathymetric surface — types and parsers (stub). |
| [s-103](iho/s-103/) | **S-103** sub-surface navigation — types and parsers (stub). |
| [s-104](iho/s-104/) | **S-104** physical environment — types and parsers (stub). |
| [s-111](iho/s-111/) | **S-111** surface currents — types and parsers (stub). |
| [s-112](iho/s-112/) | **S-112** reserved / open product slot — placeholder. |
| [s-121](iho/s-121/) | **S-121** maritime limits and boundaries — placeholder. |
| [s-122](iho/s-122/) | **S-122** marine protected areas — placeholder. |
| [s-123](iho/s-123/) | **S-123** marine radio services — placeholder. |
| [s-124](iho/s-124/) | **S-124** navigational warnings — types and parsers (stub). |
| [s-125](iho/s-125/) | **S-125** marine aids to navigation (AtoN) — placeholder. |
| [s-126](iho/s-126/) | **S-126** marine physical environment — placeholder. |
| [s-127](iho/s-127/) | **S-127** marine protected areas — types and parsers (stub). |
| [s-128](iho/s-128/) | **S-128** catalogue of nautical products — placeholder. |
| [s-129](iho/s-129/) | **S-129** under-keel clearance management — types and parsers (stub). |
| [s-130](iho/s-130/) | **S-130** polygonal demarcations of global sea areas — placeholder. |
| [s-131](iho/s-131/) | **S-131** marine harbour infrastructure — placeholder. |
| [s-164](iho/s-164/) | **S-164** test corpora — download zip bundles, discover exchange sets, parse `CATALOG.XML`. |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Workspace and each publishable crate directory include **MIT OR Apache-2.0** (`LICENSE-MIT`, `LICENSE-APACHE`) so `cargo publish` metadata matches the GitHub repository.
