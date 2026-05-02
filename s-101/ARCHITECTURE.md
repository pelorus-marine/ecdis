# Architecture: `s-101`

## Purpose

Provide **typed access** to **S-101 ENC** datasets: load **ISO 8211** exchange files, validate S-101-shaped **DDR** (presence of a **DSID** data descriptive field), ensure the first data record carries **DSID**, and expose **tag-addressable** field payloads for upcoming feature decoders.

## Current slice (implemented)

| Module | Role |
|--------|------|
| `error.rs` | [`S101Error`](src/error.rs) — `thiserror` wrapper over [`iso8211::Iso8211Error`] plus structural validation failures. |
| `edition.rs` | Pinned product specification edition ([`TARGET_PRODUCT_SPECIFICATION_EDITION`](src/edition.rs)). |
| `semantic.rs` | FRID inventory ([`FeatureInventorySummary`](src/semantic.rs)); FC XML parsing **not** implemented yet. |
| `dataset.rs` | [`S101Dataset`](src/dataset.rs) — `load`, `load_bytes`, `from_iso8211_file`, `record_count`, `first_record_dsid_payload`. |
| `decode.rs` | [`record_field_payload`](src/decode.rs) / [`field_payload`] — map directory **tags** to `user_data` bytes. |

## Boundaries

- **In scope (future):** Full feature / information types, geometry, RCID/FOID graphs, catalogue-driven validation.
- **Out of scope:** **Portrayal** (S-100 portrayal / AML); **FAFF** / permits; **Pelorus Core transports** — use [`pelorus-ecdis`](../pelorus-ecdis/) for snapshots and [`pelorus-core-adapter`](../pelorus-core-adapter/) for mapper/time scaffolding.
- **Out of scope (conformance harness):** IHO **S-164** zip corpora, exchange-set discovery, **`CATALOG.XML`** routing, GitHub release URLs, mapping manual sections or scenarios to bundles — that is **[`s-164`](../s-164/)** and **callers**.
- **Relationship to [`s-164`](../s-164/):** `s-101` accepts **ENC interchange** as paths or bytes via [`iso8211`](../iso8211/). Callers combine `s-164` output (paths/metadata) with `s-101`; **`s-101` must not depend on `s-164`.**

[`pelorus-ecdis`](../pelorus-ecdis/ARCHITECTURE.md) remains **integration** (chart + telemetry-shaped types), not the conformance-test harness layer.

## Parsing strategy

1. **Structural:** [`iso8211::DataDescriptiveFile::read`].
2. **Validation:** DDR lists a descriptive field for dataset identification (**`DSID`** short record label and **`Data Set Identification`** long field name on published exchanges); first data record includes a **`DSID`** directory tag.
3. **Semantic (next):** Interpret DSID and feature fields per IHO **S-101** edition / feature catalogue.

## Testing

- Unit tests for `decode` helpers.
- Optional **`testdata/s101_sample.000`** at workspace root: `tests/optional_fixture.rs` and [`pelorus-ecdis`](../pelorus-ecdis/) tests load it when present (CI may omit the file).

## Risks

- **Edition drift:** Pin IHO edition and FC version before expanding beyond structural checks.
- **Strict DSID rule:** Some producer quirks might require a configurable probe; revisit when real-world cells are tested.
