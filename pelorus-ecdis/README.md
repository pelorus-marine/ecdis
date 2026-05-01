# pelorus-ecdis

Bridge **[`s-101`](https://crates.io/crates/s-101) ENC** chart data with **Pelorus Core–shaped** navigation state (**own-ship**, **AIS** targets). **No CAN / NMEA stack** inside this crate.

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

## Role in the Pelorus ecosystem

[Pelorus](https://sevenseas.io/pelorus) separates **chart-grade geospatial** libraries (this repo) from **CAN FD Core** and **Ethernet Stream** transports—see the [Pelorus architecture record](https://github.com/pelorus-marine/specifications/blob/main/ARCHITECTURE.md). **`pelorus-ecdis`** is a small **integration bundle**:

- **`S101Dataset`** from **`s-101`**
- **`OwnShip`** (GNSS, COG/SOG, heading, depth — you populate from Core DCIDs upstream)
- **`AisVesselReport`** for traffic overlays

## Quick start

```toml
[dependencies]
pelorus-ecdis = "0.0.1"
```

```rust
use pelorus_ecdis::{ChartNavContext, OwnShip};
use s_101::S101Dataset;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let enc = S101Dataset::load("chart.000")?;
    let _ctx = ChartNavContext::new(enc).with_own_ship(OwnShip::with_position(51.0, 2.0));
    Ok(())
}
```

## License

**MIT OR Apache-2.0** — [LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE).

[ARCHITECTURE.md](ARCHITECTURE.md)
