# Architecture: `ecdis-runtime`

## Purpose

Thin **composition root** tying ENC parsing ([`s_101`](../s-101/)), bridge structs ([`pelorus_ecdis`](../pelorus-ecdis/)), portrayal hooks ([`ecdis_portrayal`](../ecdis-portrayal/)), behaviour stubs ([`ecdis_behaviours`](../ecdis-behaviours/)), and mapper scaffolding ([`pelorus_core_adapter`](../pelorus-core-adapter/)).

## Boundaries

- **In scope:** CLI demo binary [`main.rs`](src/main.rs).
- **Out of scope:** Network services, ENC update management, certified alarm logic — evolve in ship-side services.
