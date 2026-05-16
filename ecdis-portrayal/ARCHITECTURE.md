# Architecture: `ecdis-portrayal`

## Purpose

**Library-only** portrayal for S-101: [`PortrayalPipeline`](src/portrayal/portrayal_pipeline.rs), [`PortrayalFrame`](src/frame.rs) builders, [`DisplayMode`](src/display_mode.rs) / [`ChartTheme`](src/chart_theme.rs).

Developer Slint gallery: separate [`ecdis-portrayal-viewer`](../ecdis-portrayal-viewer/) crate (not part of this library).

## Boundaries

- **In scope:** C2IL outline CPU portrayal; catalogue palette + CSS asset reads (via [`s_101::PortrayalCatalogueBundle`](../iho/s-101/)); feature-graph geometry frames; optional symbol rasterization (`symbols` feature).
- **Out of scope:** Slint / Wayland UI; Lua AML rule execution; certified ECDIS HMI — [`ecdis-ui`](../ecdis-ui/) owns the IVI shell.
