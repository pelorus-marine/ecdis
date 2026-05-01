# ECDIS workspace — architecture index

This file is a **non-duplicative index**: each crate owns its own design narrative in **`ARCHITECTURE.md`** beneath its directory. Do not copy those details here.

## Role in the Pelorus ecosystem

This repository hosts Rust libraries for **IHO S-100-family** geospatial data and the **ISO 8211** interchange layer used by ENC and related products. It is intended as a chart-grade component in the wider [Pelorus open marine data network](https://sevenseas.io/pelorus), alongside the architecture record in [pelorus-marine/specifications](https://github.com/pelorus-marine/specifications/blob/main/ARCHITECTURE.md) (Stream-layer examples explicitly mention ECDIS connectivity). This workspace does **not** implement Pelorus Core CAN FD or Stream transports; it focuses on **parsing, validation, and type-safe access** to hydrographic exchange data suitable for an **ECDIS-class** or **chart-plotter** display chain. The **`pelorus-ecdis`** crate defines **integration-shaped** types so application services can join chart data with Core-derived telemetry without hard-coding bus protocols here.

## Intended layering

Data flows **up** from interchange bytes toward application types:

```text
ISO 8211 files / records  →  `iso8211` (structure + raw fields)
        →  `s-100` (shared S-100 model constructs, where applicable)
        →  product crates (`s-101`, `s-102`, `s-103`, …) typed features / coverages
        →  `pelorus-ecdis` (ENC + own-ship / AIS snapshot for bridge services)
        →  portrayal / full runtime (out of scope for parser crates)
```

Product crates may depend on `iso8211` and `s-100` as the workspace matures; dependency edges are documented per crate.

## Crates

Hyphenated **`s-*`** names align with **IHO S-xxx** numbering where applicable ([IHO product list](https://iho.int/en/s-100-based-product-specifications)). Each member has **`README.md`** (purpose + licensing) and an **`ARCHITECTURE.md`** for `s-*` and core tooling.

| Directory | IHO / ISO | Role |
|-----------|-----------|------|
| [`iso8211/`](iso8211/) | ISO/IEC 8211 | Binary interchange (DDR / data records). |
| [`pelorus-ecdis/`](pelorus-ecdis/) | — | [`s-101`](s-101/) + own-ship / AIS integration types. |
| [`s-61/`](s-61/) | **S-61** | Raster navigational charts (RNC)—not S-100 vector. |
| [`s-97/`](s-97/) … [`s-99/`](s-99/) | **S-97…S-99** | Guidance / interoperability / registry ops (stubs). |
| [`s-100/`](s-100/) … [`s-131/`](s-131/) | **S-100…S-131** | Framework + product slots (depth varies by crate). |
| [`s-164/`](s-164/) | **S-164** | IHO S-100 ECDIS test datasets (placeholder). |

Exact **members**: [`Cargo.toml`](Cargo.toml) `workspace.members`. **s-104** / **s-127** titles match the current IHO register (water level; marine traffic management).

**Scope note:** track work in [GitHub Issues](https://github.com/pelorus-marine/ecdis/issues); IHO/IMO texts are normative for certification.

## Repository metadata

- **License:** `MIT OR Apache-2.0` at the repository root and in each publishable crate directory (see `LICENSE-MIT` / `LICENSE-APACHE` in each crate) so `cargo publish` and the GitHub repo stay aligned.

## Where not to add design detail

- **Per-crate** `ARCHITECTURE.md` files are the source of truth for module layout, parsing strategy, and known gaps.
- **This file** should only list crates, relationships, and ecosystem positioning—**not** module tables or data-flow diagrams for individual libraries.
