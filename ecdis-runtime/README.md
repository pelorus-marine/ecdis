# ecdis-runtime

**Composition demo** binary: [`s-101`](../s-101/) load → [`pelorus-ecdis::ChartNavContext`](../pelorus-ecdis/) → [`ecdis-portrayal`](../ecdis-portrayal/) + [`ecdis-behaviours`](../ecdis-behaviours/) stubs → [`pelorus-core-adapter`](../pelorus-core-adapter/) mapper hooks.

```bash
cargo run -p ecdis-runtime -- ./path/to/cell.000
```

## License

MIT OR Apache-2.0 — [LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE).
