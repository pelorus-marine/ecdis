# ecdis-portrayal

**Portrayal library** for S-101 ECDIS stacks: pipeline traits, display modes (Day / Dusk / Night), catalogue-backed colour tokens, and UI-agnostic [`PortrayalFrame`](src/frame.rs) builders.

This crate has **no Slint dependency**. For a developer gallery UI, use the separate [`ecdis-portrayal-viewer`](../ecdis-portrayal-viewer/) crate (not used in production images). Full IVI integration remains in [`ecdis-ui`](../ecdis-ui/).

## Library

```bash
cargo build -p ecdis-portrayal
cargo test -p ecdis-portrayal
# Optional feature tests (catalogue / symbols):
cargo test -p ecdis-portrayal --features symbols,s64
```

Public API highlights:

- [`DisplayMode`](src/display_mode.rs), [`ChartTheme`](src/chart_theme.rs)
- [`PortrayalFrame`](src/frame.rs) + `build_*_frame` (including [`build_chart_frame`](src/frame.rs) for ENC C2IL + FC overlay)
- [`PortrayalCatalogueBundle`](../iho/s-101/) via `s-101` (zip SVG / `*SvgStyle.css` reads)
- [`PortrayalPipeline`](src/portrayal/portrayal_pipeline.rs)

## Feature flags

| Feature | Enables |
|---------|---------|
| `symbols` | `resvg` symbol rasterization in frame builders |
| `s64` | Load portrayal catalogue from IHO S-64 zip |

## SVG examples (headless)

```bash
cargo run -p ecdis-portrayal --example chart_preview_svg -- path/to/cell.000 out.svg
cargo run -p ecdis-portrayal --example feature_graph_preview_svg -- path/to/cell.000 path/to/S-101_FC.xml out.svg
```

## License

MIT OR Apache-2.0 — [LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE).
