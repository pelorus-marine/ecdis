# Architecture: `s-101`

## Purpose

Provide **typed access** to **S-101 ENC** datasets: load **ISO 8211** exchange files, validate S-101-shaped **DDR** (presence of a **DSID** data descriptive field), ensure the first data record carries **DSID**, and expose **tag-addressable** field payloads for upcoming feature decoders.

## Current slice (implemented)

| Module | Role |
|--------|------|
| `error.rs` | [`S101Error`](src/error.rs) — `thiserror` wrapper over [`iso8211::Iso8211Error`] plus structural validation failures. |
| `dataset.rs` | [`S101Dataset`](src/dataset.rs) — `load` / `from_iso8211_file`, `record_count`, `first_record_dsid_payload`. |
| `decode.rs` | [`record_field_payload`](src/decode.rs) / [`field_payload`] — map directory **tags** to `user_data` bytes. |

## Boundaries

- **In scope (future):** Full feature / information types, geometry, RCID/FOID graphs, catalogue-driven validation.
- **Out of scope:** **Portrayal** (S-100 portrayal / AML); **FAFF** / permits; **Pelorus Core** wiring — use [`pelorus-ecdis`](../pelorus-ecdis/) for own-ship + chart bundles.

## Parsing strategy

1. **Structural:** [`iso8211::DataDescriptiveFile::read`].
2. **Validation:** DDR lists a DDF named **`DSID`**; first data record includes a **`DSID`** directory field.
3. **Semantic (next):** Interpret DSID and feature fields per IHO **S-101** edition / feature catalogue.

## Testing

- Unit tests for `decode` helpers.
- Optional **`testdata/s101_sample.000`** at workspace root: `tests/optional_fixture.rs` and [`pelorus-ecdis`](../pelorus-ecdis/) tests load it when present (CI may omit the file).

## Risks

- **Edition drift:** Pin IHO edition and FC version before expanding beyond structural checks.
- **Strict DSID rule:** Some producer quirks might require a configurable probe; revisit when real-world cells are tested.
