#!/usr/bin/env bash
set -euo pipefail
test -f README.md
test -f latency-analysis.ipynb
grep -q ADR-042 README.md
grep -q 'p95 184 ms' README.md
echo 'synthetic R7 smoke test passed'
