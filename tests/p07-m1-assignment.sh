#!/bin/sh
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
tmp=$(mktemp -d "${TMPDIR:-/tmp}/p07-m1-assignment.XXXXXX")
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
"$root/bin/p07-m1-assignment" --out "$tmp/a.jsonl"
"$root/bin/p07-m1-assignment" --out "$tmp/b.jsonl"
cmp -s "$tmp/a.jsonl" "$tmp/b.jsonl"
jq -se '.[0].persona == "p07" and .[0].artifact_count == 200 and ([.[1:][].path]|unique|length)==200 and ([.[1:][].artifact_id]|unique|length)==200' "$tmp/a.jsonl" >/dev/null
if "$root/bin/p07-m1-assignment" --out "$tmp/a.jsonl" >/dev/null 2>&1; then exit 1; fi
ln -s "$root/workspace" "$tmp/workspace-link"
if "$root/bin/p07-m1-assignment" --out "$tmp/workspace-link/escaped.jsonl" >/dev/null 2>&1; then
  echo "symlinked protected output directory was accepted" >&2
  exit 1
fi
[ ! -e "$root/workspace/escaped.jsonl" ]
echo "p07 M1 assignment tests: ok"
