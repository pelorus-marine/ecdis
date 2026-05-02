#!/usr/bin/env bash
# Mirrors VS Code task "Fetch S-64 sample ENC": one DisplayBase S-101 cell for local UI/tests.
set -euo pipefail
ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
CACHE="${ROOT}/target/iho-cache"
mkdir -p "${CACHE}"
ZIP="${CACHE}/S-64_1.2.0.zip"
curl -fsSL -o "${ZIP}" "https://github.com/iho-ohi/S-164-Sub-Group/releases/download/v1.2.0/S-64_1.2.0.zip"
unzip -p "${ZIP}" 'S-100/DisplayBase/S100_ROOT/S-101/DATASET_FILES/10100AA_DBASE.000' > "${CACHE}/sample_enc.000"
ls -la "${CACHE}/sample_enc.000"
echo "ENC ready: ${CACHE}/sample_enc.000"
