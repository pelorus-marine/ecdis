# Architecture: `s-102`

## Purpose

Decode **IHO S-102** *Bathymetric Surface* products: regularly or irregularly gridded depth / altitude models packaged as S-100 datasets, for **ECDIS** under keel / clearance visualization and fused chart display.

## Boundaries

- **In scope (future):** Grid geometry, vertical datum references, quality / source indicators exposed as Rust types; optional resampling helpers are **application-level** unless clearly generic.
- **Out of scope:** **S-101** vector symbology; **ENC** attribution; GPU tiling — upstream of rendering.
- **Transport:** Same pattern as other products: **ISO 8211** via `iso8211`, framework metadata via `s-100`.

## Module layout (planned)

- `grid` — nodal spacing, extent, interpolation hints per product edition.
- `values` — depth / uncertainty / quality layers.
- `metadata` — dataset identification, vertical datum linkage.

## Implementation notes

- Memory footprint matters on **embedded Linux** (Yocto + Wayland / composer shell): support **streaming** or **windowed** decode before loading full global grids.

## Testing

- Validate against IHO **S-102** test datasets; include edge cases for polar / high-latitude grid definitions if applicable to your edition.
