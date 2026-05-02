# iho-testdata

**Orchestration binary:** download or open an **IHO S-164-style** test corpus zip ([`s-164`](../s-164/)), discover exchange sets, and **structurally parse** each **S-101** dataset ([`s-101`](../s-101/)) from zip members.

This crate **depends on both** libraries and stays **out of** their APIs — see [Separation of concerns](../s-164/ARCHITECTURE.md#separation-of-concerns).

## Usage

```bash
cargo run -p iho-testdata -- download
cargo run -p iho-testdata -- local ./S-64_1.2.0.zip
```

Expect **`CorruptData`** (and similar) cells to surface **`parse_fail`** — truncated ISO 8211 is intentional in those folders.

## CI integration

GitHub Actions job **`iho-testdata`** (see [`.github/workflows/ci.yml`](../.github/workflows/ci.yml)) caches the zip under `target/iho-cache/`, then runs:

```bash
cargo test -p iho-testdata s64_corpus_meets_thresholds -- --ignored
```

Bounds live in [`expectations/s64_v1_2_0.json`](expectations/s64_v1_2_0.json). `cargo test --workspace` always runs **`expectations_json_matches_schema`** so invalid JSON fails PRs without downloading the corpus.

### Local full integration

```bash
IHO_TESTDATA_ZIP=/path/to/S-64_1.2.0.zip \
  cargo test -p iho-testdata s64_corpus_meets_thresholds -- --ignored --nocapture
```

### When IHO ships a new corpus

1. Point **`IHO_S164_URL`** (and filename/path steps if needed) at the new asset in **`.github/workflows/ci.yml`**.
2. Bump **`IHO_S164_CACHE_KEY`** so CI fetches a fresh zip (cache partition).
3. Update **`pinned_*`** and numeric thresholds in **`expectations/s64_v1_2_0.json`** (or set **`IHO_EXPECTATIONS_PATH`** to a new file and commit it).

## Library API

[`CorpusRunSummary`](src/lib.rs) and [`run_corpus_zip`](src/lib.rs) are available for tests and other callers without spawning the CLI.

## License

MIT OR Apache-2.0 — [LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE).
