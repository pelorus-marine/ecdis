# Architecture: `ecdis-ui`

## Purpose

Slint-based **Wayland-facing** UI binary (`ecdis-ui`) that loads [`s_101::S101Dataset`](../s-101/), displays semantic inventory / edition pins, merges demo [`OwnShip`](../pelorus-ecdis/src/own_ship.rs) data via [`pelorus-core-adapter`](../pelorus-core-adapter/), drives [`ChartViewport`](../ecdis-portrayal/src/chart_viewport.rs) + [`PortrayalPipeline`](../ecdis-portrayal/src/lib.rs), and surfaces [`AlarmSink`](../ecdis-behaviours/src/lib.rs) events.

## Boundaries

- **In scope:** `.slint` layout under [`ui/`](ui/), Rust glue in [`src/main.rs`](src/main.rs), ENC path CLI argument.
- **Out of scope:** Certified ECDIS HMI, AML portrayal, geometry decode — evolve in `s-101` / `ecdis-portrayal` / dedicated chart engines.

## Unsafe policy

Slint-generated Rust (`build.rs` output) may contain `unsafe`; this crate does not add hand-written `unsafe` blocks.
