#!/usr/bin/env bash
set -euo pipefail
usage(){ echo "usage: rollback_beta.sh [--apply] [--help]"; }
apply=false
case "${1:-}" in --apply) apply=true;; --help) usage; exit 0;; "") ;; *) usage; exit 2;; esac
case "$(pwd)" in */repos/product-beta/docs) ;; *) echo "refusing local scope mismatch" >&2; exit 2;; esac
echo "rollback plan: disable Beta projection; preserve Alpha R7 reads; verify invariants"
if [ "$apply" = false ]; then echo "dry-run only; pass --apply after synthetic review"; exit 0; fi
[ "${SYNTHETIC_ONLY:-true}" = true ] || { echo "refusing non-synthetic mode" >&2; exit 2; }
[ -f migration-dry-run.log ] || { echo "missing evidence log" >&2; exit 2; }
echo "1/3 freeze cohort"; echo "2/3 disable projection"; echo "3/3 verify Alpha canary"
