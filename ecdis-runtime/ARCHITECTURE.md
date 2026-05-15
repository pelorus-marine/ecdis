# Architecture: `ecdis-runtime`

## Purpose

Thin **composition root** tying ENC parsing ([`s_101`](../iho/s-101/)), [`pelorus_adapter`](../../pelorus-adapter/) (`ChartNavContext` + mappers), portrayal hooks ([`ecdis_portrayal`](../ecdis-portrayal/)), and behaviour stubs ([`ecdis_behaviours`](../ecdis-behaviours/)).

## Boundaries

- **In scope:** CLI demo binary [`main.rs`](src/main.rs).
- **Out of scope:** Network services, ENC update management, certified alarm logic — evolve in ship-side services.
