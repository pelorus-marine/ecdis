# Architecture: `s-104`

## Purpose

Decode **IHO S-104** *Physical Environment* products (METOC-style data in the S-100 family) for integration with **ECDIS** and bridge systems — wind, waves, ice, and related fields per the **edition** and **product IDs** you implement.

## Boundaries

- **In scope (future):** Core coverages and metadata needed for **navigation decision support** and display overlays; explicit separation of **forecast** vs **observation** semantics where the standard distinguishes them.
- **Out of scope:** Numerical weather model ingestion from non-S-100 sources; **WMS/WFS** servers; full meteorological post-processing.

## Design considerations

- **Time series** and **multi-variable** grids may dominate memory — align buffer strategy with **Stream** delivery (see [Pelorus architecture](https://github.com/pelorus-marine/specifications/blob/main/ARCHITECTURE.md)) when datasets arrive over Ethernet vs local flash.

## Module layout (planned)

- `product` — dispatch by S-104 product identifier.
- `coverage` — shared grid / point structure handling.
- `metadata` — validity times, references, CRS.

## Testing

- Contract tests against small licensed excerpts from IHO or national hydrographic test packs.
