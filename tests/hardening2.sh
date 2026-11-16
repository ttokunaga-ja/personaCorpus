#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
export UV_CACHE_DIR="${UV_CACHE_DIR:-${TMPDIR:-/tmp}/personacorpus-uv-cache}"
export PYTHONDONTWRITEBYTECODE=1
uv run python tests/hardening2_test.py
