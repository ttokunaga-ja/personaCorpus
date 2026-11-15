#!/usr/bin/env bash
# Run every suite even if an earlier suite fails (including missing OCR tools).
set -u
cd "$(dirname "$0")/.."
export UV_CACHE_DIR="${UV_CACHE_DIR:-${TMPDIR:-/tmp}/personacorpus-uv-cache}"
status=0
for suite in tests/check.sh tests/manifest.sh tests/pack.sh tests/maketools.sh tests/batch.sh; do
  echo "=== $suite ==="
  "$suite" || status=1
done
exit "$status"
