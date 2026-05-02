# Architecture: `iho-testdata`

## Purpose

Workspace binary **`iho-testdata`**: fetch or open an **S-164-style** corpus zip ([`s-164`](../s-164/)), then **structurally parse** each **S-101** dataset discovered in exchange catalogues using **`S101Dataset::load_bytes`** ([`dataset.rs`](../s-101/src/dataset.rs)).

## Boundaries

- **Depends on:** [`s-164`](../s-164/), [`s-101`](../s-101/) only for orchestration.
- **Does not implement:** zip layout, `CATALOG.XML` semantics, or ISO 8211 — those stay in the libraries.

## Relationship

Illustrates the pattern in [s-164 separation of concerns](../s-164/ARCHITECTURE.md#separation-of-concerns): libraries stay independent; **callers** combine them.

## Testing / CI

- **Library:** [`run_corpus_zip`](src/lib.rs) / [`CorpusRunSummary`](src/lib.rs) — used by the binary and integration tests.
- **Integration:** [`tests/corpus_integration.rs`](tests/corpus_integration.rs) — **`expectations_json_matches_schema`** runs in normal `cargo test --workspace`; **`s64_corpus_meets_thresholds`** is **`#[ignore]`** and executed in CI with **`IHO_TESTDATA_ZIP`** set (see [`.github/workflows/ci.yml`](../.github/workflows/ci.yml)).
- **Adaptation:** Tune [`expectations/s64_v1_2_0.json`](expectations/s64_v1_2_0.json) when you intentionally move to another bundle or parser behaviour shifts.
