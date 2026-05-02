# meta-pelorus-ecdis

Companion **Yocto** layer for shipping [`ecdis-ui`](../../ecdis-ui/README.md) beside **Weston** on **aarch64** targets such as **NXP i.MX 95** (GPU/EGL package names come from your **meta-imx / BSP** manifest).

## Prerequisites

1. Merge **this layer** into `BBLAYERS` (`bblayers.conf`).
2. Enable **meta-rust** (or your Rust layer providing `inherit cargo`) and build **`ecdis-ui`** with [`ecdis-ui-cargo_0.0.1.bb`](recipes-ecdis/ecdis-ui/ecdis-ui-cargo_0.0.1.bb), **or** cross-compile on the host/SDK and install `/usr/bin/ecdis-ui` manually. Tune **`do_install`** source paths if your `cargo.bbclass` drops binaries elsewhere.
3. Add **`ecdis-ui`** (cargo recipe) **`ecdis-ui-launcher`** to **`IMAGE_INSTALL`** so the binary (when built in-Yocto) and systemd/Weston stubs land on the rootfs.
4. Align **Slint** licensing with product counsel before shipping firmware binaries.

### SDK / aarch64 host build

From the Yocto SDK environment (or equivalent sysroot):

```bash
rustup target add aarch64-unknown-linux-gnu
source /path/to/environment-setup-aarch64-poky-linux  # example
export PKG_CONFIG_SYSROOT_DIR="${SDKTARGETSYSROOT}"
export PKG_CONFIG_PATH="${SDKTARGETSYSROOT}/usr/lib/pkgconfig"
# Slint/bindgen often need:
export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=${SDKTARGETSYSROOT}"
cargo build -p ecdis-ui --locked --release --target aarch64-unknown-linux-gnu
```

Install the resulting `target/aarch64-unknown-linux-gnu/release/ecdis-ui` to **`/usr/bin/ecdis-ui`** on the image if you are not using the BitBake recipe.

## Recipe overview

| File | Role |
|------|------|
| [`ecdis-ui-launcher_1.0.bb`](recipes-ecdis/ecdis-ui/ecdis-ui-launcher_1.0.bb) | Ships systemd unit + `/etc/default` template + Weston notes (**does not compile Rust**). |
| [`ecdis-ui-cargo_0.0.1.bb`](recipes-ecdis/ecdis-ui/ecdis-ui-cargo_0.0.1.bb) | **`inherit cargo`** recipe sketch — validate `do_install` paths against your meta-rust revision. |
| [`ecdis-ui-cargo.bb.EXAMPLE`](recipes-ecdis/ecdis-ui/ecdis-ui-cargo.bb.EXAMPLE) | Short commented fragment for layering into custom images. |
| [`ecdis-ui.service`](recipes-ecdis/ecdis-ui/files/ecdis-ui.service) | systemd template — uses **`EnvironmentFile=-/etc/default/ecdis-ui`**. |
| [`ecdis-ui.env.example`](recipes-ecdis/ecdis-ui/files/ecdis-ui.env.example) | Installs as **`/etc/default/ecdis-ui.example`** — copy to **`/etc/default/ecdis-ui`**. |
| [`weston-ecdis-ui.snippet.ini`](recipes-ecdis/ecdis-ui/files/weston-ecdis-ui.snippet.ini) | Commented Weston launcher guidance (ENC argv ⇒ systemd or wrapper script). |

### Cross-compile notes

Slint pulls native build tools (**clang**, font stacks). Mirror **`DEPENDS`** into your BSP/SDK workflow alongside Wayland/EGL packages.

### BSP specifics (i.MX)

Wire **`RDEPENDS`** on NXP **Wayland**, **drm**, **mesa/imx-gpu** selections (`imx-gpu-viv`, etc.) per manifest naming — placeholders above stay distro-neutral.
