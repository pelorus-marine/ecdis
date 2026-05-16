# Shipping licenses (hardware / firmware)

**Not legal advice.** Product counsel should confirm this matches your distribution model.

## Policy: Path A (GPL-3.0 for `ecdis-ui`)

Pelorus ships the **`ecdis-ui`** Slint binary on embedded Linux (Yocto / Weston) under:

| Component | License when distributed |
|-----------|---------------------------|
| `ecdis-ui` binary (includes Slint) | **GPL-3.0-only** |
| Slint toolkit (user choice in Slint terms) | **GPL-3.0-only** |
| `ecdis-portrayal`, `s-101`, `iso8211`, … (source / separate libs) | **MIT OR Apache-2.0** (unchanged in git) |
| `ecdis-portrayal-viewer` | Dev host only — **not** in production `IMAGE_INSTALL` |

## Release checklist

1. **Pin versions** — Record git tag/SHA, `Cargo.lock`, and `slint` version (`1.16.1` in workspace `Cargo.toml`).
2. **Source offer** — Run from repo root:
   ```bash
   ./scripts/create-gpl-source-offer.sh <git-rev> target/gpl-source-offer-ecdis-ui.tar.xz
   ```
   Install the archive (or HTTPS URL to it) on the image under `/usr/share/doc/ecdis-ui/`.
3. **Third-party notices** — Regenerate before release:
   ```bash
   ./scripts/generate-third-party-notices.sh
   ```
   Ship `ecdis-ui/licenses/THIRD_PARTY_NOTICES` on the image (`ecdis-ui-gpl-compliance` recipe).
4. **Image packages** — `IMAGE_INSTALL` includes `ecdis-ui`, `ecdis-ui-gpl-compliance`, `ecdis-ui-launcher` (not `ecdis-portrayal-viewer`).
5. **Runtime** — Set `ECDIS_SOURCE_OFFER_URI` (see `ecdis-ui.env.example`) to the on-device path or URL shown in the About dialog.
6. **ENC data** — IHO / HO chart cells are **not** covered by this repo’s OSS licenses; track data entitlements separately.

## Yocto

| Recipe | Role |
|--------|------|
| `ecdis-ui-cargo` | Builds `ecdis-ui`; `LICENSE` includes `GPL-3.0-only`; depends on compliance package |
| `ecdis-ui-gpl-compliance` | Installs GPL text, `DISTRIBUTION.md`, third-party notices, source-offer pointer |
| `ecdis-ui-launcher` | systemd / Weston integration |

See [`yocto/meta-pelorus-ecdis/README.md`](../yocto/meta-pelorus-ecdis/README.md).

## Developer workstation

Building/running `ecdis-ui` locally for development is not a product “distribution.” The same **GPL-3.0** choice applies when you **ship** the binary to customers or on fleet hardware.
