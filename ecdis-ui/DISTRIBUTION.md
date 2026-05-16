# Distribution licensing — `ecdis-ui` binary

## Source code in git

Rust sources for this crate are **MIT OR Apache-2.0** (see `LICENSE-MIT` and `LICENSE-APACHE` in this repository). That applies to **contributions and library-style use** of the source tree.

## Shipped binary (device / firmware image)

When you build and **distribute** the `ecdis-ui` executable, it **statically links [Slint](https://slint.dev)** and other crates. This product uses **Path A**:

- **Slint** is used under the **GNU General Public License, version 3** (`GPL-3.0-only`).
- The **combined `ecdis-ui` binary** is therefore offered under **GPL-3.0-only** when distributed.

You must:

1. Provide **complete corresponding source** for the exact version you ship (see `scripts/create-gpl-source-offer.sh` and `/usr/share/doc/ecdis-ui/` on device images).
2. Include **license texts** (`GPL-3.0-only`, third-party notices) on the image or in product documentation.
3. Show an **About / License** screen in the application (built into `ecdis-ui`).

## What stays permissive

Crates such as `ecdis-portrayal`, `s-101`, and `iso8211` remain **MIT OR Apache-2.0** in source form. They are **not** relicensed to GPL unless you distribute them **only** as part of a GPL-covered combined work and choose to apply GPL to your modifications of those components in that distribution.

For policy detail and Yocto packaging, see [`docs/shipping-licenses.md`](../docs/shipping-licenses.md).
