#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
export PYTHONDONTWRITEBYTECODE=1
export UV_CACHE_DIR="${UV_CACHE_DIR:-${TMPDIR:-/tmp}/personacorpus-uv-cache}"
uv run python tests/autopilot_test.py
