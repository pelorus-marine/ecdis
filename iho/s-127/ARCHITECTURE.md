# Architecture: `s-127`

## Purpose

Decode **IHO S-127** *Marine Protected Areas* for display and alerting on ECDIS-class systems, including boundaries and regulatory metadata per the implemented **feature catalogue** edition.

## Boundaries

- **In scope (future):** MPA polygons / multiparts, identifiers, regulatory references encoded in the product.
- **Out of scope:** **Legal advice** or automatic compliance decisions; national MPA databases outside S-127 packaging.

## Module layout (planned)

- `area` — protected area features.
- `regulation` — attributes linking to restriction categories.
- `integration` — hooks for alarms or highlighting policies in the host application.

## Testing

- Prefer real-world **anonymized** excerpts; respect redistribution terms for IHO sample data.
