#!/bin/sh
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd -P)
guard="$root/bin/m1-home-manifest-guard"
tmp=$(mktemp -d "${TMPDIR:-/tmp}/m1-home-manifest-guard-test.XXXXXX")
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
home="$tmp/home"
manifest="$tmp/manifest.jsonl"
mkdir -p "$home/docs"
home=$(CDPATH= cd -- "$home" && pwd -P)
printf 'accepted\n' > "$home/docs/report.txt"
report_sha=$(shasum -a 256 "$home/docs/report.txt" | awk '{print $1}')
report_bytes=$(wc -c < "$home/docs/report.txt" | tr -d ' ')
{
  jq -cn '{schema:"persona-corpus.git-corpus-manifest/v1",kind:"header",artifact_count:1}'
  jq -cn --arg sha "$report_sha" --argjson bytes "$report_bytes" \
    '{schema:"persona-corpus.git-corpus-manifest/v1",kind:"artifact",persona:"p99",path:"workspace/people/p99-test/home/docs/report.txt",bytes:$bytes,sha256:$sha}'
} > "$manifest"

"$guard" --home "$home" --manifest "$manifest" --persona p99 --prefix workspace/people/p99-test/home/ > "$tmp/paths"
[ "$(sed -n '1p' "$tmp/paths")" = docs/report.txt ]

printf 'modified\n' > "$home/docs/report.txt"
if "$guard" --home "$home" --manifest "$manifest" --persona p99 --prefix workspace/people/p99-test/home/ >/dev/null 2>&1; then
  echo "modified accepted artifact bytes were accepted" >&2
  exit 1
fi
printf 'accepted\n' > "$home/docs/report.txt"

mkdir "$home/__pycache__"
printf 'cache\n' > "$home/__pycache__/report.cpython-314.pyc"
if "$guard" --home "$home" --manifest "$manifest" --persona p99 --prefix workspace/people/p99-test/home/ >/dev/null 2>&1; then
  echo "extra cache artifact was accepted" >&2
  exit 1
fi
rm -f "$home/__pycache__/report.cpython-314.pyc"
rmdir "$home/__pycache__"

rm -f "$home/docs/report.txt"
if "$guard" --home "$home" --manifest "$manifest" --persona p99 --prefix workspace/people/p99-test/home/ >/dev/null 2>&1; then
  echo "missing accepted artifact was accepted" >&2
  exit 1
fi

echo "m1 home manifest guard tests: ok"
