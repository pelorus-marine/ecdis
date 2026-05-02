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

published test zip  →  `s-164` (inventory / catalogue slice only)  →  caller  →  `s-101` / product crates
```

Product crates may depend on `iso8211` and `s-100` as the workspace matures; dependency edges are documented per crate.

## Conformance test corpora vs product decode

[`s-164`](s-164/) handles **packaging and routing** for published test bundles (zip, exchange-set layout, minimal `CATALOG.XML` discovery metadata). **Product crates** (`s-101`, …) handle **interchange semantics** for chart data. There is **no** intended workspace dependency edge **`s-164` → `s-101`** or **`s-101` → `s-164`**; glue belongs in binaries, tests, applications, or **[`iho-testdata`](iho-testdata/)** (example orchestration binary). Details: [s-164/ARCHITECTURE.md](s-164/ARCHITECTURE.md#separation-of-concerns).

## Crates

Hyphenated **`s-*`** names align with **IHO S-xxx** numbering where applicable ([IHO product list](https://iho.int/en/s-100-based-product-specifications)). Each member has **`README.md`** (purpose + licensing) and an **`ARCHITECTURE.md`** for `s-*` and core tooling.

| Directory | IHO / ISO | Role |
|-----------|-----------|------|
| [`iso8211/`](iso8211/) | ISO/IEC 8211 | Binary interchange (DDR / data records). |
| [`iho-testdata/`](iho-testdata/) | — | End-to-end: S-164 corpus zip → S-101 `load_bytes` (orchestration only). |
| [`pelorus-ecdis/`](pelorus-ecdis/) | — | [`s-101`](s-101/) + own-ship / AIS integration types. |
| [`s-61/`](s-61/) | **S-61** | Raster Navigational Charts (RNC)—not S-100 vector. |
| [`s-97/`](s-97/) | **S-97** | Guidelines for S-100 product specifications (stub). |
| [`s-98/`](s-98/) | **S-98** | Data product interoperability (stub). |
| [`s-99/`](s-99/) | **S-99** | GI registry operational procedures (stub). |
| [`s-100/`](s-100/) | **S-100** | Universal Hydrographic Data Model framework (stub). |
| [`s-101/`](s-101/) | **S-101** | ENC — ISO 8211 load + structural validation slice. |
| [`s-102/`](s-102/) | **S-102** | Bathymetric Surface (S-100 product; stub). |
| [`s-103/`](s-103/) | **S-103** | Sub-surface Navigation (stub). |
| [`s-104/`](s-104/) | **S-104** | Water Level Information for Surface Navigation (stub). |
| [`s-111/`](s-111/) | **S-111** | Surface Currents (stub). |
| [`s-112/`](s-112/) | **S-112** | Reserved / open product slot (stub). |
| [`s-121/`](s-121/) | **S-121** | Maritime limits and boundaries (stub). |
| [`s-122/`](s-122/) | **S-122** | Marine protected areas (stub). |
| [`s-123/`](s-123/) | **S-123** | Marine radio services (stub). |
| [`s-124/`](s-124/) | **S-124** | Navigational Warnings (stub). |
| [`s-125/`](s-125/) | **S-125** | Marine aids to navigation / AtoN (stub). |
| [`s-126/`](s-126/) | **S-126** | Marine physical environment (stub). |
| [`s-127/`](s-127/) | **S-127** | Marine Traffic Management (stub). |
| [`s-128/`](s-128/) | **S-128** | Catalogue of nautical products (stub). |
| [`s-129/`](s-129/) | **S-129** | Under Keel Clearance Management / UKCM (stub). |
| [`s-130/`](s-130/) | **S-130** | Polygonal demarcations of global sea areas (stub). |
| [`s-131/`](s-131/) | **S-131** | Marine harbour infrastructure (stub). |
| [`s-164/`](s-164/) | **S-164** | Test corpus zip download + exchange-set / `CATALOG.XML` discovery. |

Exact **members**: [`Cargo.toml`](Cargo.toml) `workspace.members` (matches table above).

**Scope note:** track work in [GitHub Issues](https://github.com/pelorus-marine/ecdis/issues); IHO/IMO texts are normative for certification.

## Repository metadata

- **License:** `MIT OR Apache-2.0` at the repository root and in each publishable crate directory (see `LICENSE-MIT` / `LICENSE-APACHE` in each crate) so `cargo publish` and the GitHub repo stay aligned.

## Where not to add design detail

- **Per-crate** `ARCHITECTURE.md` files are the source of truth for module layout, parsing strategy, and known gaps.
- **This file** should only list crates, relationships, and ecosystem positioning—**not** module tables or data-flow diagrams for individual libraries.
