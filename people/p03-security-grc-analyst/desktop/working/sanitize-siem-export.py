#!/usr/bin/env python3
"""Redact selected fields from a local, synthetic SIEM JSONL export."""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path

SENSITIVE_FIELDS = {"user", "email", "device_id", "source_address"}


def pseudonym(value: object) -> str:
    return "redacted-" + hashlib.sha256(str(value).encode()).hexdigest()[:10]


def sanitize(record: dict) -> dict:
    return {key: pseudonym(value) if key in SENSITIVE_FIELDS else value for key, value in record.items()}


def main() -> int:
    parser = argparse.ArgumentParser(description="Sanitize JSONL for audit sharing")
    parser.add_argument("input", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    lines = [json.dumps(sanitize(json.loads(line)), ensure_ascii=False) for line in args.input.read_text(encoding="utf-8").splitlines() if line.strip()]
    args.output.write_text("\n".join(lines) + ("\n" if lines else ""), encoding="utf-8")
    print(f"sanitized_records={len(lines)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
