# Architecture: `s-100`

## Purpose

Hold **cross-product S-100** types that multiple crates need without pulling in ISO 8211 or product-specific decoders. Today this means **WGS84 geometry primitives** and the **feature object identifier (FOID)** triple shared across S-100 products.

## Boundaries

- **In scope:** Small, `Copy`/`Clone`-friendly value types; pure Rust (no XML/8211).
- **Out of scope:** **ISO 8211** (see [`iso8211`](../../iso8211/)); **portrayal**; full GML/XFM binding — product crates own interchange semantics.
- **Normative source:** IHO **S-100** and registered catalogues for the edition you target.

## Module layout

| Module | Role |
|--------|------|
| [`geometry.rs`](src/geometry.rs) | [`Point2D`](src/geometry.rs), [`MultiPoint2D`](src/geometry.rs), [`Curve2D`](src/geometry.rs), [`Surface2D`](src/geometry.rs), [`Geometry`](src/geometry.rs) sum + `is_empty`. |
| [`feature_id.rs`](src/feature_id.rs) | [`FeatureObjectId`](src/feature_id.rs) — agency / fidn / fids. |
| `lib.rs` | Re-exports + [`FrameworkStub`](src/lib.rs) placeholder for early workspace wiring. |

## Relationships

- **Upstream:** Product crates (`s-101`, …) and presentation (`ecdis-portrayal`) depend on `s-100` for shared surface types.
- **Downstream:** Chart stacks compose `s-101` + `s-100` without duplicating geometry enums.

## Testing strategy

Unit tests in `lib.rs` for geometry invariants (`is_empty`, constructors).

## Risks

- **Edition drift:** Geometry here is intentionally minimal (2D WGS84 degrees); 3D / complex topologies belong in product layers until a stable shared model exists.
