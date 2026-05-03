# Contributing

Thank you for helping with the **ecdis** workspace.

## Getting started

```bash
git clone <repository-url>
cd ecdis
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

After `cargo test --workspace` succeeds:

```bash
./scripts/fetch_s64_sample_enc.sh
cargo run -p ecdis-ui --release -- target/iho-cache/sample_enc.000
```

Optional FC XML for the HUD edition line: add a second argument (path to feature catalogue XML). VS Code: tasks **Fetch S-64 sample ENC** and **rust: run ecdis-ui …**, or launch presets under [`.vscode/launch.json`](.vscode/launch.json).

Cross-compile **aarch64** with the Yocto SDK: see [`yocto/meta-pelorus-ecdis/README.md`](yocto/meta-pelorus-ecdis/README.md) (`rustup target add aarch64-unknown-linux-gnu`, `PKG_CONFIG_SYSROOT_DIR`, `BINDGEN_EXTRA_CLANG_ARGS`, etc., aligned to your sysroot).

Workspace **`release`** profile uses **`lto = "thin"`**; **`ecdis-ui`** additionally sets **`codegen-units = 1`** for a smaller binary (longer compile).

## Code style

- Run **`cargo fmt --all`** (`rustfmt.toml` at workspace root). **Keep settings aligned with** [`platform` / `dbc-rs/rustfmt.toml`](https://github.com/pelorus-marine/platform/blob/main/dbc-rs/rustfmt.toml); that tree is the Pelorus-wide rustfmt reference (this repo intentionally mirrors it).
- Use **`cargo clippy --workspace --all-targets -- -D warnings`** before committing.
- New crates follow the **`dbc-rs` ergonomics** pattern: `#![forbid(unsafe_code)]`, dual `LICENSE-*` files in the crate directory, `repository` + `license` set for `cargo publish`.

## Continuous integration (Rust / GitHub Actions)

Canonical workflow: [`.github/workflows/ci.yml`](.github/workflows/ci.yml).

- **Toolchain:** Rust **`1.90.0`** (via `dtolnay/rust-toolchain@master` + `rust-toolchain.toml`), aligned with **`platform`** / **`specifications`** for reproducible CI and local `rustfmt` / `clippy`.
- **Clippy:** `cargo clippy --workspace --all-targets -- -D warnings` **without `--all-features`** — feature combinations are exercised via normal `cargo test` and crate-local configs; full `--all-features` across the workspace is intentionally avoided here to reduce redundant / conflicting feature graphs.
- **Rustfmt:** `cargo fmt --all -- --check` — `--all` formats every package in this workspace; crates pulled in only as non-member **path** dependencies are not workspace members and are not formatted by this command.
- **Docs:** `cargo doc --workspace --no-deps` (no `--all-features`) with `RUSTDOCFLAGS=-D warnings`, consistent with clippy scope.
- **`dbc-rs` duplication:** crates from **`platform`** (`dbc-rs`, etc.) are also tested in **`platform/.github/workflows/ci.yml`** when developed there. Runs in **this** repo are for **ecdis-integration** and release hygiene; fixing the same lint in both places is normal when **`platform`** is a path dependency checkout.

## Architecture docs

- **Workspace overview:** [ARCHITECTURE.md](ARCHITECTURE.md) (index only).
- **Crate-specific design** lives in that crate’s **`ARCHITECTURE.md`** — do not copy large sections to the root file.

## Pull requests

1. Describe scope (which IHO product / crate).
2. Add tests for parsing or type changes; keep stubs compiling with at least one trivial test.
3. Close or update [GitHub issues](https://github.com/pelorus-marine/ecdis/issues) when you finish or supersede a theme. One-off bulk imports can use [`scripts/create_backlog_issues.py`](scripts/create_backlog_issues.py) (see script docstring; requires `gh auth login`).

## License

**MIT OR Apache-2.0** — see `LICENSE-MIT` and `LICENSE-APACHE` at the repo root and in each publishable crate directory.
