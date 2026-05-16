#!/usr/bin/env bash
# Create a GPL source-offer archive for a distributed ecdis-ui build (Path A).
set -euo pipefail

ROOT="$(git -C "${BASH_SOURCE[0]%/*}/.." rev-parse --show-toplevel)"
cd "${ROOT}"

REV="${1:-HEAD}"
OUT="${2:-${ROOT}/target/gpl-source-offer-ecdis-ui.tar.xz}"

if ! git rev-parse --verify "${REV}^{commit}" >/dev/null 2>&1; then
  echo "error: not a valid git revision: ${REV}" >&2
  exit 1
fi

mkdir -p "$(dirname "${OUT}")"
TMP="$(mktemp -d)"
trap 'rm -rf "${TMP}"' EXIT

mkdir -p "${TMP}/ecdis"
git -C "${ROOT}" archive "${REV}" | tar -x -C "${TMP}/ecdis"
cp "${ROOT}/Cargo.lock" "${TMP}/ecdis/Cargo.lock"

cat >"${TMP}/ecdis/SOURCE-OFFER.txt" <<EOF
Pelorus ecdis — GPL-3.0 source offer for ecdis-ui
=================================================

Git revision: ${REV}
Archive created: $(date -u +"%Y-%m-%dT%H:%M:%SZ")

The ecdis-ui binary distributed on this product is offered under GNU GPL version 3
because it links the Slint UI toolkit (https://slint.dev) under GPL-3.0-only.

This archive contains the complete corresponding source tree used to build that
binary, including Cargo.lock at the repository root.

Build (example):
  cargo build -p ecdis-ui --locked --release

See ecdis-ui/DISTRIBUTION.md and docs/shipping-licenses.md in this tree.

Slint version at time of packaging: see [workspace.dependencies] slint in Cargo.toml.
EOF

tar -C "${TMP}" -cJf "${OUT}" ecdis
echo "Wrote ${OUT} ($(du -h "${OUT}" | cut -f1))"
echo "Install on device as /usr/share/doc/ecdis-ui/gpl-source-offer-ecdis-ui.tar.xz"
echo "Set ECDIS_SOURCE_OFFER_URI=file:///usr/share/doc/ecdis-ui/gpl-source-offer-ecdis-ui.tar.xz"
