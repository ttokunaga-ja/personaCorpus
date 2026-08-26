#!/bin/sh
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
helper="$root/bin/p06-m1-assignment"
tmp=$(mktemp -d "${TMPDIR:-/tmp}/p06-m1-assignment-test.XXXXXX")
trap 'rm -rf "$tmp"' EXIT HUP INT TERM

first="$tmp/first.jsonl"
second="$tmp/second.jsonl"
"$helper" --out "$first"
"$helper" --out "$second"
cmp -s "$first" "$second"
jq -se '
  . as $rows | ($rows[0]) as $h | ($rows[1:]) as $a |
  $h.schema == "persona-corpus.m1-assignment/v1" and $h.kind == "header" and
  $h.persona == "p06" and $h.assignment_id == "p06-m1-frozen-v1" and
  $h.artifact_count == 200 and
  $h.source_assignment_sha256 == "sha256:f85b7d0687065c25257a95e39eb8ff7a4668d76e39d9f58f7ceca35afe121eaa" and
  ($a|length == 200) and ([ $a[].artifact_id ]|unique|length == 200) and
  ([ $a[].path ]|unique|length == 200) and
  (([ $a | group_by(.family)[] | {key:.[0].family,value:length} ] | from_entries) ==
   {code:6,csv_tsv:30,docx:16,domain_binary:8,html_eml:4,image:18,ipynb:6,md:12,pdf_scan:16,pdf_text:36,pptx:10,structured_text:10,txt_log:12,xlsx:16}) and
  ([ $a[] | select(.schema != "persona-corpus.m1-assignment/v1" or .kind != "artifact" or .persona != "p06" or .state != "frozen") ]|length == 0)
' "$first" >/dev/null
if "$helper" --out "$first" >/dev/null 2>&1; then
  echo "overwrite was accepted" >&2
  exit 1
fi
unsafe="$root/workspace/p06-m1-assignment-test-output.jsonl"
if "$helper" --out "$unsafe" >/dev/null 2>&1; then
  echo "unsafe workspace output was accepted" >&2
  exit 1
fi
[ ! -e "$unsafe" ]
ln -s "$root/workspace" "$tmp/workspace-link"
if "$helper" --out "$tmp/workspace-link/escaped.jsonl" >/dev/null 2>&1; then
  echo "symlinked protected output directory was accepted" >&2
  exit 1
fi
[ ! -e "$root/workspace/escaped.jsonl" ]
printf '%s\n' 'p06 M1 assignment tests: ok'
