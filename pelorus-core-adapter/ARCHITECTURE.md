# Architecture: `pelorus-core-adapter`

## Purpose

Hold **mapper traits**, **timestamp carriers**, and **light fusion helpers** between Pelorus Core / Stream payloads and [`pelorus-ecdis`](../pelorus-ecdis/) snapshot structs.

## Boundaries

- **In scope:** [`TimedOwnShip`](src/lib.rs), [`StreamInstant`](src/lib.rs), [`CoreSampleMapper`](src/lib.rs), [`merge_own_ship_fill_missing`](src/lib.rs), [`FusionClock`](src/lib.rs).
- **Out of scope:** CAN FD, socketcan, NMEA0183/2000 parsers, DBC code generation — live next to Core releases or ship services.

## Relationship

Downstream of Core schema; upstream of [`pelorus-ecdis::ChartNavContext`](../pelorus-ecdis/src/bridge.rs) population logic in application/runtime crates.
