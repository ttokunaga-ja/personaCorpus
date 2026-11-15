#!/usr/bin/env bash
set -euo pipefail
target="${1:-scratch-fixtures}"
case "$target" in scratch-fixtures|scratch-traces) ;; *) echo "refusing target: $target" >&2; exit 2;; esac
echo "dry-run: would clean local synthetic directory $target"
