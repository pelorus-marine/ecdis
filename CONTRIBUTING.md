# Contributing

Thank you for helping with the **ecdis** workspace.

## Getting started

```bash
git clone <repository-url>
cd ecdis
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
After **`cargo test --workspace`** succeeds:

```bash
./scripts/fetch_s64_sample_enc.sh
cargo run -p ecdis-ui --release -- target/iho-cache/sample_enc.000
```

Optional FC XML for the HUD edition line: add a second argument (path to feature catalogue XML). VS Code: tasks **Fetch S-64 sample ENC** and **rust: run ecdis-ui …**, or launch presets under [`.vscode/launch.json`](.vscode/launch.json).

Cross-compile **aarch64** with the Yocto SDK: see [`yocto/meta-pelorus-ecdis/README.md`](yocto/meta-pelorus-ecdis/README.md) (`rustup target add aarch64-unknown-linux-gnu`, `PKG_CONFIG_SYSROOT_DIR`, `BINDGEN_EXTRA_CLANG_ARGS`, etc., aligned to your sysroot).

Workspace **`release`** profile uses **`lto = "thin"`**; **`ecdis-ui`** additionally sets **`codegen-units = 1`** for a smaller binary (longer compile).

## Code style

- Run **`cargo fmt --all`** (`rustfmt.toml` at workspace root).
- Use **`cargo clippy --workspace --all-targets -- -D warnings`** before committing.
- New crates follow the **`dbc-rs` ergonomics** pattern: `#![forbid(unsafe_code)]`, dual `LICENSE-*` files in the crate directory, `repository` + `license` set for `cargo publish`.

## Architecture docs

- **Workspace overview:** [ARCHITECTURE.md](ARCHITECTURE.md) (index only).
- **Crate-specific design** lives in that crate’s **`ARCHITECTURE.md`** — do not copy large sections to the root file.

## Pull requests

1. Describe scope (which IHO product / crate).
2. Add tests for parsing or type changes; keep stubs compiling with at least one trivial test.
3. Close or update [GitHub issues](https://github.com/pelorus-marine/ecdis/issues) when you finish or supersede a theme. One-off bulk imports can use [`scripts/create_backlog_issues.py`](scripts/create_backlog_issues.py) (see script docstring; requires `gh auth login`).

## License

**MIT OR Apache-2.0** — see `LICENSE-MIT` and `LICENSE-APACHE` at the repo root and in each publishable crate directory.
