# ecdis-runtime

**Composition demo** binary: [`s-101`](../iho/s-101/) load → [`pelorus_adapter::ChartNavContext`](../../pelorus-adapter/) → [`ecdis-portrayal`](../ecdis-portrayal/) + [`ecdis-behaviours`](../ecdis-behaviours/) stubs + mapper hooks from the same crate.

```bash
cargo run -p ecdis-runtime -- ./path/to/cell.000
```

## License

MIT OR Apache-2.0 — [LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE).
