# Architecture: `s-103`

## Purpose

Decode **IHO S-103** *Sub-surface Navigation* datasets (S-100 product specification) for situations where **ENC (S-101)** alone is insufficient and subsurface-related navigation information is distributed as an S-100 exchange set.

## Boundaries

- **In scope (future):** Product records as defined by the normative S-103 edition in use, on top of `iso8211` transport (same pattern as `s-101`).
- **Out of scope:** **S-61** raster navigational charts; **dynamic own-ship** sensors (belongs in Pelorus Core / [`pelorus-ecdis`](../pelorus-ecdis/)).

## Dependencies (planned)

- `iso8211` workspace path.
- Optional shared types in `s-100` when multiple products need the same helpers.

## Testing

- IHO or HO-supplied **S-103** samples under compatible license; re-use the optional `testdata/` pattern if appropriate.
