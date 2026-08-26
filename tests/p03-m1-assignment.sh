#!/bin/sh
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
helper="$root/bin/p03-m1-assignment"
tmp=$(mktemp -d "${TMPDIR:-/tmp}/p03-m1-assignment-test.XXXXXX")
trap 'rm -rf "$tmp"' EXIT HUP INT TERM

first="$tmp/first.jsonl"
second="$tmp/second.jsonl"
# This regression intentionally runs against the accepted post-Full home.  A
# helper that still requires home == 200 M1 files must fail here.
[ "$(find "$root/workspace/people/p03-security-grc-analyst/home" -type f | wc -l | tr -d ' ')" -gt 200 ]
"$helper" --out "$first"
"$helper" --out "$second"
cmp -s "$first" "$second"
jq -se '
  . as $rows | ($rows[0]) as $h | ($rows[1:]) as $a |
  $h.schema == "persona-corpus.m1-assignment/v1" and $h.kind == "header" and
  $h.persona == "p03" and $h.assignment_id == "p03-m1-frozen-v1" and
  $h.artifact_count == 200 and
  $h.source_assignment_sha256 == "sha256:30dd24c19194ff422501bd1fb31f46cefa10a455c295028bfe904d9936233bbc" and
  ($a|length == 200) and ([ $a[].artifact_id ]|unique|length == 200) and
  ([ $a[].path ]|unique|length == 200) and
  ($a[0].artifact_id == "p03-m1-0001") and
  (($a | group_by(.family) | map({key:.[0].family,value:length}) | from_entries) ==
   {code:16,csv_tsv:20,docx:10,domain_binary:6,html_eml:16,image:6,md:20,pdf_scan:10,pdf_text:30,pptx:4,structured_text:30,txt_log:24,xlsx:8}) and
  (([ $a[] | select(.artifact_id == "p03-m1-0092") ] | .[0]) ==
   {schema:"persona-corpus.m1-assignment/v1",kind:"artifact",persona:"p03",artifact_id:"p03-m1-0092",scope_id:"p03-primary-12",scope_path:"meetings/security-reviews",family:"txt_log",physical_extension:"txt",path:"meetings/security-reviews/security-review-action-log.txt",state:"frozen"}) and
  (([ $a[] | select(.artifact_id == "p03-m1-0094") ] | .[0]) ==
   {schema:"persona-corpus.m1-assignment/v1",kind:"artifact",persona:"p03",artifact_id:"p03-m1-0094",scope_id:"p03-primary-12",scope_path:"meetings/security-reviews",family:"pptx",physical_extension:"pptx",path:"meetings/security-reviews/vendor-risk-quarterly-review.pptx",state:"frozen"}) and
  ([ $a[] | select(.schema != "persona-corpus.m1-assignment/v1" or .kind != "artifact" or .persona != "p03" or .state != "frozen") ]|length == 0)
' "$first" >/dev/null
if "$helper" --out "$first" >/dev/null 2>&1; then
  echo "overwrite was accepted" >&2
  exit 1
fi
unsafe="$root/workspace/p03-m1-assignment-test-output.jsonl"
if "$helper" --out "$unsafe" >/dev/null 2>&1; then
  echo "unsafe workspace output was accepted" >&2
  exit 1
fi
[ ! -e "$unsafe" ] || { echo "unsafe output was created" >&2; exit 1; }
symlink_out="$tmp/symlink-output.jsonl"
ln -s "$first" "$symlink_out"
if "$helper" --out "$symlink_out" >/dev/null 2>&1; then
  echo "symlink output was accepted" >&2
  exit 1
fi

echo "p03 M1 assignment tests: ok"
