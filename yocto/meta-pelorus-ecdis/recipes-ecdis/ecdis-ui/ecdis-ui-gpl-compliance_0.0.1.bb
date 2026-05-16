SUMMARY = "GPL-3.0 compliance files for Pelorus ecdis-ui (Path A)"
DESCRIPTION = "License texts, third-party notices, and source-offer pointer for distributed ecdis-ui binaries."
LICENSE = "MIT | Apache-2.0 & GPL-3.0-only"
LIC_FILES_CHKSUM = " \
    file://${S}/ecdis-ui/LICENSE-MIT;md5=e1aa2d983f3785b02342740aabe5b7d9 \
    file://${S}/ecdis-ui/LICENSE-APACHE;md5=f424aae774b9431ce9d3f5be7c7de503 \
    file://${S}/ecdis-ui/licenses/GPL-3.0-only.txt;md5=8da5784ab1c72e63ac74971f88658166 \
"

SRC_URI = "git://github.com/pelorus-marine/ecdis.git;protocol=https;branch=main"
SRCREV ?= "${AUTOREV}"
PV = "0.0.1+git${SRCPV}"

S = "${WORKDIR}/git"

# Optional: add a pre-built source-offer tarball to SRC_URI and install below.
# file://gpl-source-offer-ecdis-ui.tar.xz

do_configure[noexec] = "1"
do_compile[noexec] = "1"

do_install() {
    install -d ${D}${datadir}/doc/ecdis-ui
    install -m0644 ${S}/ecdis-ui/DISTRIBUTION.md ${D}${datadir}/doc/ecdis-ui/
    install -m0644 ${S}/ecdis-ui/licenses/GPL-3.0-only.txt ${D}${datadir}/doc/ecdis-ui/
    install -m0644 ${S}/ecdis-ui/licenses/THIRD_PARTY_NOTICES ${D}${datadir}/doc/ecdis-ui/

    install -d ${D}${sysconfdir}/default
    printf '%s\n' \
        '# Shown in ecdis-ui About / License (override in /etc/default/ecdis-ui)' \
        'ECDIS_SOURCE_OFFER_URI=file://${datadir}/doc/ecdis-ui/source-offer.txt' \
        > ${D}${datadir}/doc/ecdis-ui/source-offer.env

    printf '%s\n' \
        'Pelorus ecdis-ui — GPL-3.0 source offer' \
        '' \
        'Corresponding source for the ecdis-ui binary on this image is offered under' \
        'GNU General Public License version 3 (GPL-3.0-only), because the program links Slint.' \
        '' \
        'Obtain the complete buildable source tree (git revision + Cargo.lock) from your' \
        'image maintainer or build host using:' \
        '  scripts/create-gpl-source-offer.sh <git-rev> gpl-source-offer-ecdis-ui.tar.xz' \
        '' \
        'When installed on this image, the archive should be placed at:' \
        '  /usr/share/doc/ecdis-ui/gpl-source-offer-ecdis-ui.tar.xz' \
        '' \
        'Set ECDIS_SOURCE_OFFER_URI to that file:// URI (see ecdis-ui.env.example).' \
        > ${D}${datadir}/doc/ecdis-ui/source-offer.txt
}

FILES:${PN} = " \
    ${datadir}/doc/ecdis-ui/* \
"

RDEPENDS:${PN} = ""
