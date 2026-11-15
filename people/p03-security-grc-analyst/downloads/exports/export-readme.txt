Cobalt Harbor Systems — Defensive export package
===============================================

Purpose / 目的
--------------
This directory contains a fully synthetic example of a minimized audit export.
It supports the Aegis Control AC-27 evidence-handling exercise dated 2026-07-13
for evidence set 184 and the fictional risk record RISK-031. No live systems,
accounts, personal data, secrets, or operational endpoints are involved.

Contents / 内容
--------------
* export-redactor.py: turns local synthetic labels into a small redacted manifest.
* manifest-normalizer.rs: sorts and de-duplicates command-line labels offline.
* export-bundle-index.json: describes the simulated bundle and its retention rule.
* redaction-profile.yaml: reviewable minimization decisions in Japanese and English.

Safe review sequence / 安全な確認手順
---------------------------------
1. Read the JSON index and YAML profile.
2. Use only short, invented labels when exercising either utility.
3. Verify that the generated records contain review tokens instead of sensitive labels.
4. Remove any local test output after the tabletop exercise.

This material is for defensive assurance training and audit documentation only.
