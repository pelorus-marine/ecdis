# Architecture: `s-100`

## Purpose

Model **IHO S-100** framework concepts in Rust so **product specification** crates (`s-101`, `s-102`, …) share a single, versioned interpretation of the Universal Hydrographic Data Model (UHDM) — **without** embedding full GML/XFM plumbing on day one.

## Boundaries

- **In scope (future):** identifiers aligned with the S-100 GML schema family, common record / catalogue patterns, reusable error types, and thin abstractions that product crates can build on.
- **Out of scope:** **ISO 8211** byte parsing (see [`iso8211`](../iso8211/)); **portrayal** (S-100 Portrayal / AML); full **XML/GML** document DOM — add separate crates if needed.
- **Normative source:** IHO S-100 main document and registered **feature / portrayal catalogues** for the edition you target; this repository must name the edition explicitly once implementation begins.

## Module layout (planned)

No stable layout yet. Likely evolution:

- `model` — core identifiers, optional attribute carriers.
- `catalogue` — feature catalogue and portrayal references (read-only first).
- `error` — shared `thiserror`-style errors when dependencies justify them.

Today the crate is a **stub** with a single placeholder type so the workspace and publishes resolve.

## Relationships

- **Upstream:** Product crates should depend on `s-100`, not duplicate framework enums.
- **Downstream:** Application / ECDIS integration builds on `s-101` + optional `s-102`… crates after decoders exist.

## Testing strategy (future)

- Golden files from IHO test data sets (where redistribution is permitted).
- Round-trip or snapshot tests per **product** crate; `s-100` should stay covered by **unit tests** for pure type invariants only.

## Risks

- **Edition drift:** S-100 evolves; pin editions in crate metadata and document migration.
- **Over-modeling early:** prefer thin types and explicit version gates over a single rigid hierarchy.
