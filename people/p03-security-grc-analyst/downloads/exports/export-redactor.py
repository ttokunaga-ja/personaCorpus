#!/usr/bin/env python3
"""Create a safe, reviewable redacted export manifest.

This offline utility only normalizes a supplied list of synthetic evidence labels.
It never opens network connections or reads credentials.
"""
from __future__ import annotations

import hashlib
import json
import re
import sys
from pathlib import Path
from typing import Iterable

SENSITIVE_LABEL = re.compile(r"(?i)(email|owner|case|identifier)")


def redact_label(label: str) -> str:
    """Replace sensitive-looking label values with a stable review token."""
    normalized = " ".join(label.split())
    if not normalized:
        raise ValueError("label must not be empty")
    if SENSITIVE_LABEL.search(normalized):
        digest = hashlib.sha256(normalized.encode("utf-8")).hexdigest()[:12]
        return f"[REDACTED:{digest}]"
    return normalized


def build_manifest(labels: Iterable[str]) -> dict[str, object]:
    records = [{"source_label": redact_label(label)} for label in labels]
    return {
        "profile": "AC-27 export minimization / Aegis review",
        "records": records,
        "notice": "Synthetic, defensive audit artifact; no personal data is retained.",
    }


def main(argv: list[str]) -> int:
    if len(argv) != 3:
        print("usage: export-redactor.py INPUT.txt OUTPUT.json", file=sys.stderr)
        return 2
    source, target = map(Path, argv[1:])
    labels = source.read_text(encoding="utf-8").splitlines()
    target.write_text(json.dumps(build_manifest(labels), ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
