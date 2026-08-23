#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "$0")/.." && pwd)
fixture_root=$(mktemp -d "${TMPDIR:-/tmp}/persona-corpus-manifest.XXXXXX")
trap 'rm -rf "$fixture_root"' EXIT

mkdir -p "$fixture_root/canonical" \
  "$fixture_root/workspace/people/p99-test-persona/home/documents" \
  "$fixture_root/manifests"
printf '%s\n' '{"schema":"test","personas":[{"id":"p99","role":"test-persona","scopes":[{"id":"p99-primary-01","path":"documents"}]}]}' > "$fixture_root/canonical/persona-plan.json"
printf 'alpha\n' > "$fixture_root/workspace/people/p99-test-persona/home/documents/alpha.txt"

"$repo_root/bin/corpus-manifest" generate \
  --repo-root "$fixture_root" \
  --manifest manifests/corpus-sha256.jsonl
"$repo_root/bin/corpus-manifest" verify \
  --repo-root "$fixture_root" \
  --manifest manifests/corpus-sha256.jsonl

printf 'changed\n' >> "$fixture_root/workspace/people/p99-test-persona/home/documents/alpha.txt"
if "$repo_root/bin/corpus-manifest" verify \
  --repo-root "$fixture_root" \
  --manifest manifests/corpus-sha256.jsonl >/dev/null 2>&1; then
  echo "expected changed artifact verification to fail" >&2
  exit 1
fi

printf 'alpha\n' > "$fixture_root/workspace/people/p99-test-persona/home/documents/alpha.txt"
ln -s alpha.txt "$fixture_root/workspace/people/p99-test-persona/home/documents/link.txt"
if "$repo_root/bin/corpus-manifest" generate \
  --repo-root "$fixture_root" \
  --manifest manifests/symlink.jsonl >/dev/null 2>&1; then
  echo "expected symlink rejection" >&2
  exit 1
fi
rm "$fixture_root/workspace/people/p99-test-persona/home/documents/link.txt"

ln "$fixture_root/workspace/people/p99-test-persona/home/documents/alpha.txt" \
  "$fixture_root/workspace/people/p99-test-persona/home/documents/hardlink.txt"
if "$repo_root/bin/corpus-manifest" generate \
  --repo-root "$fixture_root" \
  --manifest manifests/hardlink.jsonl >/dev/null 2>&1; then
  echo "expected hard-link rejection" >&2
  exit 1
fi
rm "$fixture_root/workspace/people/p99-test-persona/home/documents/hardlink.txt"

truncate -s 104857601 \
  "$fixture_root/workspace/people/p99-test-persona/home/documents/oversize.bin"
if "$repo_root/bin/corpus-manifest" generate \
  --repo-root "$fixture_root" \
  --manifest manifests/oversize.jsonl >/dev/null 2>&1; then
  echo "expected oversized blob rejection" >&2
  exit 1
fi
rm "$fixture_root/workspace/people/p99-test-persona/home/documents/oversize.bin"

if "$repo_root/bin/corpus-manifest" generate \
  --repo-root "$fixture_root" \
  --manifest manifests/raised-limit.jsonl \
  --max-blob-bytes 104857601 >/dev/null 2>&1; then
  echo "expected raised blob limit rejection" >&2
  exit 1
fi

mkdir -p "$fixture_root/workspace/people/p99-test-persona/not-home"
printf 'outside\n' > "$fixture_root/workspace/people/p99-test-persona/not-home/outside.txt"
if "$repo_root/bin/corpus-manifest" generate \
  --repo-root "$fixture_root" \
  --manifest manifests/outside-home.jsonl >/dev/null 2>&1; then
  echo "expected outside-home rejection" >&2
  exit 1
fi
rm "$fixture_root/workspace/people/p99-test-persona/not-home/outside.txt"
rmdir "$fixture_root/workspace/people/p99-test-persona/not-home"

mkdir -p "$fixture_root/workspace/people/p98-unplanned/home/documents"
printf 'unplanned\n' > "$fixture_root/workspace/people/p98-unplanned/home/documents/unplanned.txt"
if "$repo_root/bin/corpus-manifest" generate \
  --repo-root "$fixture_root" \
  --manifest manifests/unplanned.jsonl >/dev/null 2>&1; then
  echo "expected unplanned persona rejection" >&2
  exit 1
fi
rm "$fixture_root/workspace/people/p98-unplanned/home/documents/unplanned.txt"
rmdir "$fixture_root/workspace/people/p98-unplanned/home/documents"
rmdir "$fixture_root/workspace/people/p98-unplanned/home"
rmdir "$fixture_root/workspace/people/p98-unplanned"

if "$repo_root/bin/corpus-manifest" generate \
  --repo-root "$fixture_root" \
  --manifest "$fixture_root/outside.jsonl" >/dev/null 2>&1; then
  echo "expected out-of-root manifest rejection" >&2
  exit 1
fi

echo "corpus-manifest tests passed"
