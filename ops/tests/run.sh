#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
exec cargo test --release --manifest-path "$root/tools/Cargo.toml" --test ops "$@"
