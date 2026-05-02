# Builds `ecdis-ui` with meta-rust `cargo` bbclass. Paths (`cargo.bbclass` revision,
# ${WORKDIR}/build vs ${B}, cross-target triple) vary by Yocto release — validate with `bitbake -e`.
SUMMARY = "Pelorus ECDIS Slint UI binary (Rust cargo)"
LICENSE = "MIT | Apache-2.0"
LIC_FILES_CHKSUM = " \
    file://${S}/LICENSE-MIT;md5=e1aa2d983f3785b02342740aabe5b7d9 \
    file://${S}/LICENSE-APACHE;md5=f424aae774b9431ce9d3f5be7c7de503 \
"

inherit cargo

SRC_URI = "git://github.com/pelorus-marine/ecdis.git;protocol=https;branch=main"
SRCREV ?= "${AUTOREV}"
PV = "0.0.1+git${SRCPV}"

S = "${WORKDIR}/git"

CARGO_BUILD_FLAGS = "--locked --profile release --package ecdis-ui"

# Slint native codegen / bindgen commonly wants a host toolchain; extend per BSP.
DEPENDS += "clang-native llvm-native pkgconfig-native"

# Runtime: adjust Wayland/EGL/GPU package names for meta-imx / your BSP (e.g. imx-gpu-viv).
RDEPENDS:${PN} += " \
    wayland-client \
    libxkbcommon \
    fontconfig \
"

FILES:${PN} = "${bindir}/ecdis-ui"

do_install() {
    install -d ${D}${bindir}
    BIN=""
    for cand in $(find "${WORKDIR}" -path '*/release/ecdis-ui' -type f 2>/dev/null); do
        BIN="$cand"
        break
    done
    if [ -z "$BIN" ] || [ ! -f "$BIN" ]; then
        bbfatal "ecdis-ui binary not found under ${WORKDIR} — adjust do_install for your cargo.bbclass layout"
    fi
    install -m 0755 "$BIN" ${D}${bindir}/ecdis-ui
}