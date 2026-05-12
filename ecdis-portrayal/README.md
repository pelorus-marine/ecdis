# ecdis-portrayal

Stub **[`PortrayalPipeline`](src/lib.rs)** for S-101 charts — AML / symbol libraries **not** included yet.

## Quick visualization (SVG)

From an ENC `.000` and matching **S-101 feature catalogue** XML, render **`s_101::FeatureGraph`**
geometry (same 880×420 viewbox as `ecdis-ui`). **Use real paths** — `path/to/...` below is only a shape hint:

```text
cargo run -p ecdis-portrayal --example feature_graph_preview_svg -- \
  /path/to/cell.000 /path/to/S-101_FC.xml out.svg
```

After you have cached **S-64** (`Corpus::fetch_default` or manual download), you can extract inputs and run:

```text
unzip -p ~/.cache/pelorus-marine/s-164/S-64_1.2.0.zip \
  'S-100/DisplayStandard/S100_ROOT/S-101/DATASET_FILES/10100AA_STNDR.000' > /tmp/stndr.000
unzip -p ~/.cache/pelorus-marine/s-164/S-64_1.2.0.zip \
  'S-100/InitialCatalogues/S100_ROOT/S-101/CATALOGUES/S-101_1.0.2_20220524.xml' > /tmp/s101_fc.xml
cargo run -p ecdis-portrayal --example feature_graph_preview_svg -- \
  /tmp/stndr.000 /tmp/s101_fc.xml /tmp/graph.svg
```

Optional 4th argument: scale denominator (default `22000`). Open `out.svg` in a browser.

**C2IL-only** outline (no FC file):

```text
cargo run -p ecdis-portrayal --example chart_preview_svg -- /path/to/cell.000 out.svg
```

## License

MIT OR Apache-2.0 — [LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE).
