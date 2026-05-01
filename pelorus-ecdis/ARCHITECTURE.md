# Architecture: `pelorus-ecdis`

## Purpose

Provide a **thin integration layer** between:

1. **Chart data** — [`s_101::S101Dataset`] from this workspace (ISO 8211 + S-101 validation).
2. **Own-ship and traffic** — values ultimately sourced from **Pelorus Core** (and possibly Stream), expressed here as plain Rust structs.

This keeps the **data plane** (parsers) separate from the **vehicle bus**, matching the layering in the [Pelorus architecture record](https://github.com/pelorus-marine/specifications/blob/main/ARCHITECTURE.md).

## Non-goals

- No `socketcan`, NMEA, or **DBC** bindings here.
- No portrayal, route monitoring, or COLREG automation.
- No persistence or ENC update policy.

Those belong in application or higher-level Pelorus services.

## Types

| Type | Role |
|------|------|
| [`OwnShip`](src/own_ship.rs) | Lat/lon, COG/SOG, heading, depth — fill from Core talkers upstream. |
| [`AisVesselReport`](src/ais.rs) | Sparse AIS target for overlays; not a full VDM decode model. |
| [`ChartNavContext`](src/bridge.rs) | Owns an [`S101Dataset`] plus dynamic snapshots. |

## Future work

- Optional `Stream`-flavored timestamps on [`OwnShip`] for latency-aware fusion.
- Alignment tables documenting **DCID → field** mapping (lives in Pelorus specs or `dbc-rs` docs, referenced from here).
- C API (`cbindgen`) only if a non-Rust stack requires it on Yocto — defer until product needs it.

## Testing

Tests that require a chart load **skip** when `testdata/s101_sample.000` is absent, same pattern as [`s-101`](../s-101/).
