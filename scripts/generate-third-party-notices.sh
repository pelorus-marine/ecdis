#!/usr/bin/env bash
# Regenerate ecdis-ui third-party license listing for GPL distribution (Path A).
set -euo pipefail

ROOT="$(git -C "${BASH_SOURCE[0]%/*}/.." rev-parse --show-toplevel)"
OUT="${ROOT}/ecdis-ui/licenses/THIRD_PARTY_NOTICES"
HEADER="${ROOT}/ecdis-ui/licenses/THIRD_PARTY_NOTICES.header"

cd "${ROOT}"

if ! command -v cargo-license >/dev/null 2>&1; then
  echo "Installing cargo-license (user local)…" >&2
  cargo install cargo-license --locked 2>/dev/null || true
fi

if ! command -v cargo-license >/dev/null 2>&1; then
  echo "error: cargo-license not found; install with: cargo install cargo-license" >&2
  exit 1
fi

{
  cat "${HEADER}"
  echo ""
  echo "Generated: $(date -u +"%Y-%m-%dT%H:%M:%SZ")"
  echo "Package: ecdis-ui (release dependency closure)"
  echo ""
  cargo license \
    --manifest-path "${ROOT}/ecdis-ui/Cargo.toml" \
    --avoid-dev-deps \
    --avoid-build-deps \
    --json \
    | python3 -c "
import json, sys
rows = json.load(sys.stdin)
for r in sorted(rows, key=lambda x: (x.get('license') or '', x.get('name') or '')):
    name = r.get('name', '?')
    ver = r.get('version', '?')
    lic = r.get('license', 'UNKNOWN')
    print(f'{name} {ver} — {lic}')
"
} >"${OUT}"

echo "Wrote ${OUT}"
