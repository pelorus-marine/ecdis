# Architecture: `ecdis-portrayal`

## Purpose

Define **[`PortrayalPipeline`](src/lib.rs)** hooks binding [`s_101::S101Dataset`](../iho/s-101/) to future AML/portrayal engines, plus **[`ChartViewport`](src/chart_viewport.rs)** for mariner pan/zoom state tied to scale denominators.

## Boundaries

- **In scope:** trait surface + [`NoPortrayal`](src/lib.rs) stub; [`demo_stub_segments_px`](src/chart_viewport.rs) UI stubs (non-navigation-grade projection).
- **Out of scope:** S-100 AML execution, GPU contexts, symbol catalogs — application-owned.
