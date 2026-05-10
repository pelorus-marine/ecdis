# Pelorus ECDIS — architecture record

**Last Updated:** May 2, 2026  
**Status:** Living (non-normative)

## 1. Project

### Mission

**Chart-grade** Rust libraries for **IHO S-100-family** geospatial products and the **ISO 8211** interchange layer used by ENC and related hydrographic exchange. The workspace supplies **parsing, validation, and type-safe access** to data suitable for an **ECDIS-class** or **chart-plotter** display chain, with **integration-shaped** types so bridge services can join chart products with Pelorus-derived telemetry **without** embedding bus protocols in every crate.

### Relationship to Pelorus specifications

Full programme mission, [**Legacy Marine Data Ecosystem (LMDE)**](https://github.com/pelorus-marine/specifications/blob/main/ARCHITECTURE.md#lmde), and subsystem definitions (**Core**, **Stream**, **State**) live in the [Specifications architecture record](https://github.com/pelorus-marine/specifications/blob/main/ARCHITECTURE.md). This repository **does not** implement Pelorus **Core** (CAN FD) or **Stream** transports; it aligns with the ecosystem where **Stream** examples explicitly mention **ECDIS connectivity**—here as **data and portrayal**, not as a replacement for nautical truth on the Core fieldbus.

The **`pelorus-ecdis`** crate holds **integration-shaped** boundaries toward **`pelorus-core`** via **`pelorus-core-adapter`** (mapper traits and timestamps—**no sockets** in-tree).

### Presence

- [Pelorus project site](https://sevenseas.io/pelorus) — Landing page for the Pelorus open marine data network.
- [Specifications repository](https://github.com/pelorus-marine/specifications) — Programme-wide architecture and DCID / Stream drafts.
- [ECDIS workspace repository](https://github.com/pelorus-marine/ecdis) — Source of truth for this document set; issues and chart-stack changes live here.
- [Seven Seas community](https://sevenseas.io/) — Project-facing brand and wider community entry point.

---

## 2. Problems this workspace targets

Weaknesses in **ENC / S-100 toolchain friction** and **chart-application integration** that this codebase addresses:

- **Opaque interchange stacks**: ISO 8211 and product XML/binary bundles are hard to navigate without typed, testable Rust boundaries—this workspace splits **structure** (`iso8211`), **framework** (`s-100`), and **product crates** (`s-101`, …) so callers progress deliberately.
- **Conformance vs exploration**: Published **S-164** test corpora and vendor bundles need **routing** separate from **semantic decode**—[`s-164`](s-164/) packages discovery; **product crates** own interchange semantics (see [Conformance corpora vs product decode](#conformance-corpora-vs-product-decode) below).
- **Bridge integration gaps**: Operators want **chart context** and **own-ship / AIS** snapshots aligned with Pelorus **DCID**-style contracts—**`pelorus-ecdis`** and **`pelorus-core-adapter`** define those seams **without** dragging Core or Stream sockets into every chart crate.
- **Presentation portability**: Display stacks differ (CLI demo, Slint IVI shell)—**`ecdis-portrayal`** / **`ecdis-behaviours`** isolate portrayal and behaviour stubs from interchange decoding.

**Pelorus ECDIS** does **not** claim **IMO type approval** or bit-identical behaviour vs any vendor ECDIS kernel—those remain matters for integrators, classification societies, and normative IHO/IMO texts.

---

## 3. Subsystems

### Intended layering

Data flows **up** from interchange bytes toward application types:

```text
ISO 8211 files / records  →  `iso8211` (structure + raw fields)
        →  `s-100` (shared S-100 model constructs, where applicable)
        →  product crates (`s-101`, `s-102`, `s-103`, …) typed features / coverages
        →  `pelorus-ecdis` (ENC + own-ship / AIS snapshot for bridge services)
        →  `pelorus-core-adapter` (Core/Stream mapper traits + timestamps — no sockets)
        →  `ecdis-portrayal` / `ecdis-behaviours` (presentation + nav-behaviour stubs)
        →  `ecdis-runtime` (CLI composition demo) / `ecdis-ui` (Slint Wayland IVI shell)

published test zip  →  `s-164` (inventory / catalogue slice only)  →  caller  →  `s-101` / product crates
```

Product crates may depend on `iso8211` and `s-100` as the workspace matures; dependency edges are documented per crate.

### Non-duplicative detail

Each crate may carry its own **`ARCHITECTURE.md`** beneath its directory for module layout, parsing strategy, and known gaps. **This file** lists ecosystem position, layers, and crate index—not low-level module tables.

### Crates

Hyphenated **`s-*`** names align with **IHO S-xxx** numbering where applicable ([IHO product list](https://iho.int/en/s-100-based-product-specifications)). Each member has **`README.md`** (purpose + licensing) and an **`ARCHITECTURE.md`** for `s-*` and core tooling where present.

| Directory | IHO / ISO | Role |
|-----------|-----------|------|
| [`iso8211/`](iso8211/) | ISO/IEC 8211 | Binary interchange (DDR / data records). |
| [`iho-testdata/`](iho-testdata/) | — | End-to-end: S-164 corpus zip → S-101 `load_bytes` (orchestration only). |
| [`ecdis-behaviours/`](ecdis-behaviours/) | — | ECDIS behaviour stubs (alarms, overscale predicate). |
| [`ecdis-portrayal/`](ecdis-portrayal/) | — | Portrayal trait + [`ChartViewport`](ecdis-portrayal/src/chart_viewport.rs) stubs; AML execution TBD. |
| [`ecdis-runtime/`](ecdis-runtime/) | — | Composition-root demo (`ecdis-runtime` binary). |
| [`ecdis-ui/`](ecdis-ui/) | — | Slint ENC HUD + stub chart for Weston/Yocto targets — [`README`](ecdis-ui/README.md). |
| [`pelorus-core-adapter/`](pelorus-core-adapter/) | — | Core/Stream sample → `pelorus-ecdis` snapshots (types only). |
| [`pelorus-ecdis/`](pelorus-ecdis/) | — | [`s-101`](s-101/) + own-ship / AIS integration types. |
| [`s-61/`](s-61/) | **S-61** | Raster Navigational Charts (RNC)—not S-100 vector. |
| [`s-97/`](s-97/) | **S-97** | Guidelines for S-100 product specifications (stub). |
| [`s-98/`](s-98/) | **S-98** | Data product interoperability (stub). |
| [`s-99/`](s-99/) | **S-99** | GI registry operational procedures (stub). |
| [`s-100/`](s-100/) | **S-100** | Universal Hydrographic Data Model framework (stub). |
| [`s-101/`](s-101/) | **S-101** | ENC — ISO 8211 load + FRID inventory / edition pins (FC XML decode TBD). |
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

**Scope note:** track work in [GitHub Issues](https://github.com/pelorus-marine/ecdis/issues); **IHO** / **IMO** normative texts govern certification—this repository is implementation and integration scaffolding.

### Conformance corpora vs product decode

[`s-164`](s-164/) handles **packaging and routing** for published test bundles (zip, exchange-set layout, minimal `CATALOG.XML` discovery metadata). **Product crates** (`s-101`, …) handle **interchange semantics** for chart data. There is **no** intended workspace dependency edge **`s-164` → `s-101`** or **`s-101` → `s-164`**; glue belongs in binaries, tests, applications, or **[`iho-testdata`](iho-testdata/)** (example orchestration binary). Details: [s-164/ARCHITECTURE.md](s-164/ARCHITECTURE.md#separation-of-concerns).

---

## 4. Trademarks and third-party names

Pelorus ECDIS is an independent open-source workspace. **This is not legal advice**; consult counsel before shipping product packaging, marketing, or certifications that cite hydrographic or classification regimes.

**IHO**, **S-100**, **ENC**, product numbers (**S-101**, **S-164**, …), and related hydrographic programme names are cited **nominatively** to identify standards and test artefacts; **rights belong to the International Hydrographic Organization and its partners**, not this repository.

Commercial marine networks and programmes discussed in the [Specifications architecture record](https://github.com/pelorus-marine/specifications/blob/main/ARCHITECTURE.md#5-trademarks-and-third-party-names) (**NMEA**, **OneNet**, OEM buses, etc.) remain **third-party** marks there—this workspace does **not** imply wire-level compatibility with any incumbent ECDIS product line unless a normative conformance document and tests establish it.

---

## Repository metadata

- **License:** `MIT OR Apache-2.0` at the repository root and in each publishable crate directory (see `LICENSE-MIT` / `LICENSE-APACHE` in each crate) so `cargo publish` and the GitHub repo stay aligned.

## Where not to add design detail

- **Per-crate** `ARCHITECTURE.md` files are the source of truth for module layout, parsing strategy, and known gaps.
- **This file** should only list crates, relationships, and ecosystem positioning—**not** module tables or data-flow diagrams duplicated from individual libraries.
