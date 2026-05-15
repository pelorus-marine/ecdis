# Architecture: `s-111`

## Purpose

Implement **IHO S-111** *Surface Currents* for **vector or gridded** current representations used on ECDIS and integrated bridge systems.

## Boundaries

- **In scope (future):** Current components, temporal validity, spatial referencing compatible with **S-100** common patterns.
- **Out of scope:** Hydrodynamic simulation; tide models unless encoded as S-111 product content.

## Module layout (planned)

- `model` — feature / coverage interpretation per edition.
- `time` — validity windows and forecast reference times.
- `units` — speed / direction conventions.

## Embedded / IVI notes

- Prefer **lazy** decode of tiles or subregions for **Weston**-based compositors with constrained RAM under Yocto.
