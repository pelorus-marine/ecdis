# Architecture: `s-129`

## Purpose

Decode **IHO S-129** *Under Keel Clearance Management* datasets that support **UKC** monitoring and alarm logic on ECDIS when combined with draft inputs, tide models, and bathymetry (**S-102** / ENC depth).

## Boundaries

- **In scope (future):** Dataset structure for UKC management policies encoded by the product; exposure of parameters needed by a **safety-of-navigation** runtime (implemented in application code, not this crate alone).
- **Out of scope:** **ARPA/AIS** sensor fusion; autopilot commands; classification society approval logic.

## Cross-crate dependencies (planned)

- Expect **composition** with `s-101` (ENC) and `s-102` (bathymetric surface) in the host application; this crate supplies **interpreted S-129 content** only.

## Module layout (planned)

- `management` — UKC management container / metadata.
- `policy` — encoded parameters and thresholds per edition.
- `error` — structured validation for safety-critical UI layers.

## Embedded notes

- UKC computation may run **periodically** on a low-power bridge computer (Yocto, **Weston**, minimal **IVI-style** shell): avoid heap spikes during full-chart refresh; expose **incremental** updates.
