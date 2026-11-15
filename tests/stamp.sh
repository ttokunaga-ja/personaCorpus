#!/usr/bin/env bash
set -u
cd "$(dirname "$0")/.."
export UV_CACHE_DIR="${UV_CACHE_DIR:-${TMPDIR:-/tmp}/personacorpus-uv-cache}"
export PYTHONDONTWRITEBYTECODE=1
uv run python tests/stamp_test.py "$@"
