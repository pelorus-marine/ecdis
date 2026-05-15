# Architecture: `s-124`

## Purpose

Decode **IHO S-124** *Navigational Warnings* so **ECDIS** can list, filter, and highlight warnings consistently with the S-100 data model (rather than only raster or plain-text overlays).

## Boundaries

- **In scope (future):** Warning areas, text, status lifecycle, references to NAVAREA sources as defined by the implemented edition.
- **Out of scope:** **GMDSS** radio stack; **official** promulgation authority workflows; automatic acceptance of warnings for **SOLAS** compliance without OEM policy.

## Implementation notes

- **Update cadence** may be **high** — API should allow incremental merge of new exchange sets (aligns with Pelorus **Stream** connectivity for downloads in the field).

## Module layout (planned)

- `warning` — core warning information record.
- `geometry` — polygons / polylines for affected sea areas.
- `catalogue` — feature / attribute binding per IHO FC.
