# ecdis-portrayal-viewer

**Developer-only** Slint app for inspecting [`ecdis-portrayal`](../ecdis-portrayal/) frames (C2IL outline, feature graph, symbol gallery, theme swatches). Not built or shipped with production ECDIS images — use [`ecdis-ui`](../ecdis-ui/) for the IVI demo.

Requires Slint platform libraries (see [`ecdis-ui/README.md`](../ecdis-ui/README.md)).

```bash
cargo run -p ecdis-portrayal-viewer -- path/to/cell.000 [S-101_FC.xml] \
  [--s64-zip target/iho-cache/S-64_1.2.0.zip] \
  [--portrayal-catalogue=path/to/PC.zip] \
  [--display-mode=day|dusk|night]
```

## License

MIT OR Apache-2.0 — same as the workspace.
