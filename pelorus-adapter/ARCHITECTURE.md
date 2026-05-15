# Architecture: `pelorus-adapter`

## Purpose

Single **integration layer** between:

1. **Chart data** — [`s_101::S101Dataset`] (ISO 8211 + S-101 validation).
2. **Own-ship and traffic** — [`OwnShip`], [`AisVesselReport`], bundled in [`ChartNavContext`].
3. **Core/Stream mapping** — [`CoreSampleMapper`], [`TimedOwnShip`], [`StreamInstant`], [`merge_own_ship_fill_missing`], [`FusionClock`].

Keeps parsers (`s-101`, `iso8211`) separate from the vehicle bus, matching the [Pelorus architecture record](https://github.com/pelorus-marine/specifications/blob/main/ARCHITECTURE.md).

## Non-goals

- No `socketcan`, NMEA, or **DBC** bindings.
- No portrayal, route monitoring, or COLREG automation.
- No persistence or ENC update policy.

Those belong in application or higher-level Pelorus services (`ecdis-ui`, `ecdis-runtime`, ship services).

## Types

| Type | Module | Role |
|------|--------|------|
| [`ChartNavContext`](src/bridge.rs) | `bridge` | `Arc<S101Dataset>` + own-ship + AIS + VDR status line. |
| [`OwnShip`](src/own_ship.rs) | `own_ship` | Lat/lon, COG/SOG, heading, depth — from Core or manual fill. |
| [`AisVesselReport`](src/ais.rs) | `ais` | Sparse AIS overlay target. |
| [`CoreSampleMapper`](src/mapper.rs) | `mapper` | Trait: opaque payload → timed snapshots. |
| [`UnconfiguredMapper`](src/mapper.rs) | `mapper` | Stub until DCID bindings exist upstream. |

## Relationship to other crates

- **Downstream of** [`pelorus-core`](../../platform/pelorus-core) schema (`OwnShipSnapshot` re-exported).
- **Upstream of** [`ecdis-ui`](../ecdis-ui/), [`ecdis-runtime`](../ecdis-runtime/) — they build `ChartNavContext` and wire mappers.

## Future work

- DCID → field alignment tables (Pelorus specs / `dbc-rs`).
- C API (`cbindgen`) only if a non-Rust stack requires it on Yocto.

## Testing

[`bridge.rs`](src/bridge.rs) tests skip when `testdata/s101_sample.000` is absent, same as [`s-101`](../iho/s-101/).
