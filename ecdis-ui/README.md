# ecdis-ui

**Slint** shell binary for loading **S-101** ENC `.000` cells and exercising [`ecdis-portrayal`](../ecdis-portrayal/) viewport hooks plus [`ecdis-behaviours`](../ecdis-behaviours/) alarm sinks.

For portrayal-only debugging (display modes, symbol gallery, theme swatches), use the dev-only gallery: `cargo run -p ecdis-portrayal-viewer` — see [`ecdis-portrayal-viewer/README.md`](../ecdis-portrayal-viewer/README.md).

Chart geometry uses a fixed **880×420** logical viewbox (scaled to the chart rectangle): ENC **C2IL** polylines render as blue strokes; **demo stub** appears when the cell has no decodable chains. An **orange cross** marks own-ship when latitude/longitude fall inside the canvas (same projection as outlines).

### Run (developer workstation)

Requires Slint’s Linux build dependencies (Fontconfig, EGL/GL, Wayland client libs, etc. — see [Slint Rust setup](https://docs.slint.dev/latest/docs/rust/slint)).

```bash
cargo run -p ecdis-ui --release -- /path/to/cell.000
# Optional FC XML path (edition triple shown in HUD):
cargo run -p ecdis-ui --release -- /path/to/cell.000 /path/to/feature_catalogue.xml
```

Use **About** in the UI for GPL-3.0 distribution notice and source-offer location.

### Own-ship demo inputs (env / CLI)

Without overrides, the demo uses the same defaults as before (51°N 2°E, SOG from 3 m/s, HDG 42°, depth 8.5 m; COG unset). Set **`PELORUS_OWNSHIP_*`** environment variables and/or optional flags (after the `.000` / XML arguments) to drive [`OwnShipSnapshot`](../../platform/pelorus-core/src/ownship/snapshot.rs) through to the chart:

| Variable | Optional CLI flag | Meaning |
|----------|-------------------|---------|
| `PELORUS_OWNSHIP_LAT` | `--ownship-lat=` | Degrees north |
| `PELORUS_OWNSHIP_LON` | `--ownship-lon=` | Degrees east |
| `PELORUS_OWNSHIP_COG` | `--ownship-cog=` | Course over ground (° true); if unset, COG stays empty |
| `PELORUS_OWNSHIP_SOG_KN` | `--ownship-sog-kn=` | Speed over ground (knots) |
| `PELORUS_OWNSHIP_HDG` | `--ownship-hdg=` | Heading (° true) |
| `PELORUS_OWNSHIP_DEPTH_M` | `--ownship-depth-m=` | Depth (m) |

Example: `PELORUS_OWNSHIP_LAT=50.7 PELORUS_OWNSHIP_LON=-1.0 cargo run -p ecdis-ui -- …/cell.000 --ownship-hdg=90`

Use zoom/pan buttons, **drag** on the chart (touch or mouse), or **scroll/wheel** on the chart for zoom. Logs use **`tracing`**; set e.g. `RUST_LOG=ecdis.nav=debug`.

### Sample ENC (IHO S-64)

From the repo root, either run the VS Code task **“Fetch S-64 sample ENC”** or:

```bash
./scripts/fetch_s64_sample_enc.sh
cargo run -p ecdis-ui --release -- target/iho-cache/sample_enc.000
```

### Weston

Run Weston (nested session or hardware), ensure `WAYLAND_DISPLAY` is exported in that shell, then start `ecdis-ui` from the same environment. Use your compositor or kiosk shell for fullscreen chrome (Slint’s public Rust API here does not toggle fullscreen).

## Composition note

The HUD mirrors [`pelorus_adapter::ChartNavContext`](../../pelorus-adapter/) (**ENC via `Arc<S101Dataset>`**, own-ship snapshot, AIS vector stub): [`OwnShip`](../../pelorus-adapter/src/own_ship.rs) is built from a runtime [`OwnShipSnapshot`](../../pelorus-adapter/src/lib.rs) (env/CLI in this binary; production would use [`CoreSampleMapper`](../../pelorus-adapter/src/mapper.rs) or a live Core path). Chart outlines come from [`CpuOutlinePortrayal`](../ecdis-portrayal/src/cpu_outline.rs) when the cell carries **C2IL** chains; otherwise the demo stub segments are shown.

## Licensing

| Artifact | License |
|----------|---------|
| **Source in git** (this crate’s Rust files) | **MIT OR Apache-2.0** (`LICENSE-MIT`, `LICENSE-APACHE`) |
| **Distributed `ecdis-ui` binary** (links Slint) | **GPL-3.0-only** — [DISTRIBUTION.md](DISTRIBUTION.md) |

Slint is used under **GPL-3.0-only** (Path A). Before shipping firmware:

1. Read [docs/shipping-licenses.md](../docs/shipping-licenses.md)
2. Run `./scripts/create-gpl-source-offer.sh <git-rev>`
3. Run `./scripts/generate-third-party-notices.sh`
4. Install compliance files on the image (`ecdis-ui-gpl-compliance` Yocto recipe)

Override source-offer location: `ECDIS_SOURCE_OFFER_URI` (default `file:///usr/share/doc/ecdis-ui/source-offer.txt`).

## See also

- Yocto layer: [`yocto/meta-pelorus-ecdis/`](../yocto/meta-pelorus-ecdis/README.md)
