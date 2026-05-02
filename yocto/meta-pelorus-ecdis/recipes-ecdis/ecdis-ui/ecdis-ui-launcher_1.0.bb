SUMMARY = "Weston/systemd integration stubs for Pelorus ecdis-ui"
DESCRIPTION = "Installs unit files and Weston launcher snippets. Build the ecdis-ui Rust binary separately (Yocto SDK / meta-rust) and place it at /usr/bin/ecdis-ui — see layer README."
HOMEPAGE = "https://github.com/pelorus-marine/ecdis"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=083985ade698c0b850d761cbbefbdc27"

SRC_URI = " \
    file://ecdis-ui.service \
    file://ecdis-ui.env.example \
    file://weston-ecdis-ui.snippet.ini \
"

S = "${WORKDIR}"

inherit systemd

SYSTEMD_SERVICE:${PN} = "ecdis-ui.service"
SYSTEMD_AUTO_ENABLE = "disable"

do_install() {
    install -d ${D}${systemd_system_unitdir}
    install -m0644 ${WORKDIR}/ecdis-ui.service ${D}${systemd_system_unitdir}/ecdis-ui.service

    install -d ${D}${sysconfdir}/default
    install -m0644 ${WORKDIR}/ecdis-ui.env.example ${D}${sysconfdir}/default/ecdis-ui.example

    install -d ${D}${datadir}/pelorus-ecdis
    install -m0644 ${WORKDIR}/weston-ecdis-ui.snippet.ini ${D}${datadir}/pelorus-ecdis/weston-ecdis-ui.snippet.ini
}

FILES:${PN} += " \
    ${systemd_system_unitdir}/ecdis-ui.service \
    ${sysconfdir}/default/ecdis-ui.example \
    ${datadir}/pelorus-ecdis/weston-ecdis-ui.snippet.ini \
"
