# Architecture: `ecdis-behaviours`

## Purpose

House incremental **ECDIS behaviour** helpers (alarms, overscale predicate) separate from parsers.

## Boundaries

- **In scope:** [`display_is_overscaled_vs_chart_minimum`](src/lib.rs), [`AlarmSink`](src/lib.rs), [`NavAlertKind`](src/lib.rs).
- **Out of scope:** IEC 61174 certification evidence, route monitoring geometry, ENC update policies — extend deliberately.
