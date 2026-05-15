# pelorus-adapter

Bridge **[`s-101`](https://crates.io/crates/s-101) ENC** chart data with **Pelorus Core–shaped** navigation state (**own-ship**, **AIS**), plus **Core/Stream mapper traits** and fusion helpers. **No CAN / NMEA stack** in this crate.

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

## Role in the Pelorus ecosystem

[Pelorus](https://sevenseas.io/pelorus) separates **chart-grade geospatial** libraries (this repo) from **CAN FD Core** and **Ethernet Stream** transports—see the [Pelorus architecture record](https://github.com/pelorus-marine/specifications/blob/main/ARCHITECTURE.md). **`pelorus-adapter`** provides:

- **`ChartNavContext`** — [`S101Dataset`](https://crates.io/crates/s-101) plus dynamic own-ship / AIS snapshots
- **`OwnShip`**, **`AisVesselReport`** — plain structs you populate from Core DCIDs
- **`CoreSampleMapper`**, **`TimedOwnShip`**, **`merge_own_ship_fill_missing`** — map opaque Core/Stream payloads without sockets in-tree

## Quick start

```toml
[dependencies]
pelorus-adapter = "0.0.1"
```

```rust
use pelorus_adapter::{ChartNavContext, CoreSampleMapper, OwnShip, UnconfiguredMapper};
use s_101::S101Dataset;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let enc = S101Dataset::load("chart.000")?;
    let _ctx = ChartNavContext::new(enc).with_own_ship(OwnShip::with_position(51.0, 2.0));
    let _ = CoreSampleMapper::map_own_ship(&UnconfiguredMapper, &[]);
    Ok(())
}
```

## License

**MIT OR Apache-2.0** — [LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE).

[ARCHITECTURE.md](ARCHITECTURE.md)
