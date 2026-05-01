# ecdis

Pelorus **ECDIS** / **S-100** Rust workspace: **ISO 8211** interchange (`iso8211`) and **IHO S-100 family** product libraries (`s-100`, `s-101`, …).

See [ARCHITECTURE.md](ARCHITECTURE.md) for the workspace index (links to each crate’s `ARCHITECTURE.md` — **no duplicated design detail** at the root). Work backlog: [GitHub Issues](https://github.com/pelorus-marine/ecdis/issues).

**Recent progress:** `s-101` loads ISO 8211 and validates DSID shape; `pelorus-ecdis` bundles `S101Dataset` with `OwnShip` / AIS snapshots; `iso8211` `DataRecord` exposes `field_tags` aligned with fields; legacy non-hyphen crates.io tombstones live in sibling [`pelorus-legacy-stubs`](https://github.com/pelorus-marine/pelorus-legacy-stubs).

**Ecosystem:** This repo is a chart-grade component in the [Pelorus](https://sevenseas.io/pelorus) program; the high-level system picture lives in the [Pelorus architecture record](https://github.com/pelorus-marine/specifications/blob/main/ARCHITECTURE.md).

## Repository layout

```text
ecdis/
  Cargo.toml
  ARCHITECTURE.md
  scripts/           # e.g. create_backlog_issues.py
  testdata/          # optional s101_sample.000 — see testdata/README.md
  iso8211/
  pelorus-ecdis/     # S-101 + own-ship / AIS integration types (no CAN stack in-crate)
  s-100/ … s-129/    # IHO products (s-103 = sub-surface navigation; s-101 = ENC slice)
```

Full crate table: [ARCHITECTURE.md](ARCHITECTURE.md).

## Prerequisites

- Rust **stable** (see `rust-toolchain.toml`: `rustfmt`, `clippy`)

## Common commands

```bash
# Format
cargo fmt --all

# Test entire workspace
cargo test --workspace --verbose

# Lint
cargo clippy --workspace --all-targets -- -D warnings

# Docs (warnings as errors)
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --open

# Example: ISO 8211 structure dump
cargo run -p iso8211 --example print -- path/to/file.000
```

## Crate docs

| Crate | README | Architecture |
|-------|--------|--------------|
| iso8211 | [iso8211/README.md](iso8211/README.md) | [iso8211/ARCHITECTURE.md](iso8211/ARCHITECTURE.md) |
| pelorus-ecdis | [pelorus-ecdis/README.md](pelorus-ecdis/README.md) | [pelorus-ecdis/ARCHITECTURE.md](pelorus-ecdis/ARCHITECTURE.md) |
| s-100 … s-129 | `README.md` in each directory | `ARCHITECTURE.md` in each directory |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Workspace and each publishable crate directory include **MIT OR Apache-2.0** (`LICENSE-MIT`, `LICENSE-APACHE`) so `cargo publish` metadata matches the GitHub repository.
