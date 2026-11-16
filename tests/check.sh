#!/usr/bin/env bash
set -u
cd "$(dirname "$0")/.."
export UV_CACHE_DIR="${UV_CACHE_DIR:-${TMPDIR:-/tmp}/personacorpus-uv-cache}"
uv run python tests/tools_fixtures.py check
first=$?
uv run python tests/check_regression_test.py
second=$?
if (( first || second )); then exit 1; fi
