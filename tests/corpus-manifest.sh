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

# --tracked-only selects the Git index, not concurrent untracked production
# output.  A staged change is included, but an unstaged change is fail-closed.
git -C "$fixture_root" init -q
git -C "$fixture_root" config user.email manifest-test@example.invalid
git -C "$fixture_root" config user.name manifest-test
git -C "$fixture_root" add canonical/persona-plan.json \
  workspace/people/p99-test-persona/home/documents/alpha.txt
printf 'staged\n' > "$fixture_root/workspace/people/p99-test-persona/home/documents/alpha.txt"
git -C "$fixture_root" add workspace/people/p99-test-persona/home/documents/alpha.txt
printf 'untracked\n' > "$fixture_root/workspace/people/p99-test-persona/home/documents/untracked.txt"

printf '%s\n' '{"schema":"drift","personas":[]}' > "$fixture_root/canonical/persona-plan.json"
if "$repo_root/bin/corpus-manifest" generate \
  --repo-root "$fixture_root" \
  --manifest manifests/tracked-plan-drift.jsonl \
  --tracked-only >/dev/null 2>&1; then
  echo "expected tracked-only manifest to reject unstaged canonical plan drift" >&2
  exit 1
fi
git -C "$fixture_root" checkout -- canonical/persona-plan.json

"$repo_root/bin/corpus-manifest" generate \
  --repo-root "$fixture_root" \
  --manifest manifests/tracked-only.jsonl \
  --tracked-only
"$repo_root/bin/corpus-manifest" verify \
  --repo-root "$fixture_root" \
  --manifest manifests/tracked-only.jsonl \
  --tracked-only
if ! rg -F '"artifact_count":1' "$fixture_root/manifests/tracked-only.jsonl" >/dev/null; then
  echo "expected tracked-only manifest to include exactly the staged artifact" >&2
  exit 1
fi
if rg -F 'untracked.txt' "$fixture_root/manifests/tracked-only.jsonl" >/dev/null; then
  echo "expected tracked-only manifest to exclude untracked artifact" >&2
  exit 1
fi
if rg -F '"selection"' "$fixture_root/manifests/tracked-only.jsonl" >/dev/null; then
  echo "expected tracked-only manifest bytes to remain mode-independent" >&2
  exit 1
fi

# Default mode remains a live-tree manifest and therefore includes the same
# untracked addition that tracked-only intentionally excludes.
"$repo_root/bin/corpus-manifest" generate \
  --repo-root "$fixture_root" \
  --manifest manifests/default-live-tree.jsonl
if ! rg -F '"artifact_count":2' "$fixture_root/manifests/default-live-tree.jsonl" >/dev/null; then
  echo "expected default live-tree manifest to include untracked artifact" >&2
  exit 1
fi
if ! rg -F 'untracked.txt' "$fixture_root/manifests/default-live-tree.jsonl" >/dev/null; then
  echo "expected default live-tree manifest to retain existing behavior" >&2
  exit 1
fi
rm "$fixture_root/workspace/people/p99-test-persona/home/documents/untracked.txt"
"$repo_root/bin/corpus-manifest" verify \
  --repo-root "$fixture_root" \
  --manifest manifests/tracked-only.jsonl
"$repo_root/bin/corpus-manifest" generate \
  --repo-root "$fixture_root" \
  --manifest manifests/default-same-snapshot.jsonl
if ! cmp -s "$fixture_root/manifests/tracked-only.jsonl" \
  "$fixture_root/manifests/default-same-snapshot.jsonl"; then
  echo "expected tracked-only and default manifests to match for the same snapshot" >&2
  exit 1
fi

printf 'unstaged drift\n' > "$fixture_root/workspace/people/p99-test-persona/home/documents/alpha.txt"
if "$repo_root/bin/corpus-manifest" generate \
  --repo-root "$fixture_root" \
  --manifest manifests/tracked-drift.jsonl \
  --tracked-only >/dev/null 2>&1; then
  echo "expected tracked-only manifest to reject unstaged content drift" >&2
  exit 1
fi
git -C "$fixture_root" checkout -- workspace/people/p99-test-persona/home/documents/alpha.txt

rm "$fixture_root/canonical/persona-plan.json"
if "$repo_root/bin/corpus-manifest" generate \
  --repo-root "$fixture_root" \
  --manifest manifests/tracked-plan-missing.jsonl \
  --tracked-only >"$fixture_root/plan-missing.out" 2>&1; then
  echo "expected tracked-only manifest to reject missing indexed canonical plan" >&2
  exit 1
fi
if rg -F 'Traceback' "$fixture_root/plan-missing.out" >/dev/null; then
  echo "expected missing indexed canonical plan to return a handled error" >&2
  exit 1
fi
git -C "$fixture_root" checkout -- canonical/persona-plan.json

# A selected indexed symlink and a selected worktree hardlink are both unsafe.
ln -s alpha.txt "$fixture_root/workspace/people/p99-test-persona/home/documents/indexed-link.txt"
git -C "$fixture_root" add workspace/people/p99-test-persona/home/documents/indexed-link.txt
if "$repo_root/bin/corpus-manifest" generate \
  --repo-root "$fixture_root" \
  --manifest manifests/tracked-symlink.jsonl \
  --tracked-only >/dev/null 2>&1; then
  echo "expected tracked-only manifest to reject indexed symlink" >&2
  exit 1
fi
git -C "$fixture_root" rm --cached -q workspace/people/p99-test-persona/home/documents/indexed-link.txt
rm "$fixture_root/workspace/people/p99-test-persona/home/documents/indexed-link.txt"
ln "$fixture_root/workspace/people/p99-test-persona/home/documents/alpha.txt" \
  "$fixture_root/workspace/people/p99-test-persona/home/documents/selected-hardlink.txt"
git -C "$fixture_root" add workspace/people/p99-test-persona/home/documents/selected-hardlink.txt
if "$repo_root/bin/corpus-manifest" generate \
  --repo-root "$fixture_root" \
  --manifest manifests/tracked-hardlink.jsonl \
  --tracked-only >/dev/null 2>&1; then
  echo "expected tracked-only manifest to reject selected hardlink" >&2
  exit 1
fi
rm "$fixture_root/workspace/people/p99-test-persona/home/documents/selected-hardlink.txt"
mkfifo "$fixture_root/workspace/people/p99-test-persona/home/documents/selected-hardlink.txt"
if "$repo_root/bin/corpus-manifest" generate \
  --repo-root "$fixture_root" \
  --manifest manifests/tracked-fifo.jsonl \
  --tracked-only >/dev/null 2>&1; then
  echo "expected tracked-only manifest to reject selected non-regular file" >&2
  exit 1
fi

echo "corpus-manifest tests passed"
