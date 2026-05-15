# Architecture: `s-101`

## Purpose

Provide **typed access** to **S-101 ENC** datasets: load **ISO 8211** exchange files, validate S-101-shaped **DDR** (presence of a **DSID** data descriptive field), ensure the first data record carries **DSID**, classify **data records** into typed structs, parse **feature catalogue** XML (subset), and build a **feature graph** (FOID-keyed features with FC-resolved class/attributes and WGS84 geometry via [`s-100`](../s-100/)).

## Current slice (implemented)

| Module | Role |
|--------|------|
| `error.rs` | [`S101Error`](src/error.rs) — `thiserror` wrapper over [`iso8211::Iso8211Error`] plus structural validation failures. |
| `edition.rs` | Pinned product specification edition ([`TARGET_PRODUCT_SPECIFICATION_EDITION`](src/edition.rs)). |
| `binary.rs` | Low-level field trimming / little-endian reads shared by record parsers. |
| `dataset.rs` | [`S101Dataset`](src/dataset.rs) — `load`, `load_bytes`, `typed_records`, `integer_crs_parameters`, `build_feature_graph`. |
| `decode.rs` | [`record_field_payload`](src/decode.rs) / [`field_payload`] — map directory **tags** to `user_data` bytes. |
| `record/` | Per-category ISO 8211 record decoders (`DSID`, `CSID`, `PRID`, `MRID`/`C3IL`, curves, surfaces, features, …) + [`Record`](src/record/mod.rs) classifier. |
| `fc/` | [`FeatureCatalogue::parse_xml`](src/fc/catalogue.rs) — simple attributes, feature types, information types (subset adequate for graph resolution). |
| `graph.rs` | [`FeatureGraph`](src/graph.rs) — resolve `SPAS` → spatial records → [`s_100::Geometry`](../s-100/src/geometry.rs). |
| `geometry.rs` | Integer CRS helpers (`DSSI` / coordinate system) for WGS84 conversion. |
| `portrayal_catalog.rs` | S-101 portrayal catalogue (zip + manifest) for AML stage 2 hooks. |
| `semantic.rs` | FRID inventory ([`FeatureInventorySummary`](src/semantic.rs)); raw `FRID` iterators for stage-1 portrayal. |

## Boundaries

- **In scope:** Typed spatial + feature records for S-64 v1.2.0–observed layouts; FC-driven class/attribute decode; straight-segment curve geometry; surfaces from ring references.
- **Out of scope:** **Portrayal Lua** execution (see [`ecdis-portrayal`](../ecdis-portrayal/)); **FAFF** / permits; **Pelorus Core transports** — use [`pelorus-adapter`](../../pelorus-adapter/) for snapshots and mapper/time scaffolding.
- **Out of scope (conformance harness):** IHO **S-164** zip corpora, exchange-set discovery, **`CATALOG.XML`** routing — that is **[`s-164`](../s-164/)** and **callers**.
- **Relationship to [`s-164`](../s-164/):** `s-101` accepts **ENC interchange** as paths or bytes via [`iso8211`](../../iso8211/). Callers combine `s-164` output (paths/metadata) with `s-101`; **`s-101` must not depend on `s-164`.**

[`pelorus-adapter`](../../pelorus-adapter/ARCHITECTURE.md) remains **integration** (chart + telemetry-shaped types), not the conformance-test harness layer.

## Parsing strategy

1. **Structural:** [`iso8211::DataDescriptiveFile::read`].
2. **Validation:** DDR lists **DSID**; first data record includes **DSID**.
3. **Semantic:** `typed_records()` classifies by directory tags; unknown rows become [`Record::Unknown`](src/record/mod.rs). `build_feature_graph` indexes spatial records and resolves `SPAS` + integer CRS to [`s_100::Geometry`](../s-100/src/geometry.rs).

## Testing

- Unit tests for `decode` / `binary` helpers.
- Optional **`testdata/s101_sample.000`** at workspace root: `tests/optional_fixture.rs` and [`pelorus-adapter`](../../pelorus-adapter/) tests load it when present (CI may omit the file).
- **`tests/s164_corpus_integration.rs`** (`#[ignore]` unless **`IHO_TESTDATA_ZIP`** is set): full **S-164** zip via [`s_164::Corpus`](../s-164/) — `FeatureCatalogue::parse_xml` + `build_feature_graph` over every positive S-101 cell. CI runs **`decodes_and_summarises_corpus_from_local_zip`** against a cached **S-64** zip (see [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml)).

## Risks

- **Edition drift:** Pin IHO edition and FC version before expanding beyond structural checks.
- **Strict DSID rule:** Some producer quirks might require a configurable probe; revisit when real-world cells are tested.
- **Partial FC XML:** Associations / complex-attribute trees are not fully modelled; unresolved values surface as [`AttributeValue::Raw`](src/graph.rs).
