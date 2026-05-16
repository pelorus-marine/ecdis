# ecdis-portrayal-viewer

**Developer-only** Slint app for inspecting [`ecdis-portrayal`](../ecdis-portrayal/) frames: **ENC chart** (C2IL coastline outlines, same as `ecdis-ui`), feature graph (FC geometry, C2IL-aligned when the cell has chains), symbol gallery, and theme swatches. Pan/zoom on the chart pane. Not built or shipped with production ECDIS images — use [`ecdis-ui`](../ecdis-ui/) for the IVI demo.

Requires Slint platform libraries (see [`ecdis-ui/README.md`](../ecdis-ui/README.md)).

```bash
cargo run -p ecdis-portrayal-viewer -- path/to/cell.000 [S-101_FC.xml] \
  [--s64-zip target/iho-cache/S-64_1.2.0.zip] \
  [--portrayal-catalogue=path/to/PC.zip] \
  [--display-mode=day|dusk|night]
```

## License

**Source:** MIT OR Apache-2.0 (same as the workspace).

**Do not ship this binary on production hardware** — use `ecdis-ui` with [GPL-3.0 distribution compliance](../docs/shipping-licenses.md) or keep this tool on developer workstations only.
