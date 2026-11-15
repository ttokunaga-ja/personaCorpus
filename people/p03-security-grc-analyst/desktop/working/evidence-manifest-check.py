#!/usr/bin/env python3
"""Offline evidence-manifest verifier for a controlled audit workspace."""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(65536), b""):
            digest.update(chunk)
    return digest.hexdigest()


def main() -> int:
    parser = argparse.ArgumentParser(description="Verify local evidence checksums")
    parser.add_argument("manifest", type=Path, help="JSON file with path and sha256 fields")
    args = parser.parse_args()
    entries = json.loads(args.manifest.read_text(encoding="utf-8"))
    failures = []
    for entry in entries:
        path = args.manifest.parent / entry["path"]
        actual = sha256(path) if path.is_file() else None
        if actual != entry["sha256"]:
            failures.append({"path": entry["path"], "expected": entry["sha256"], "actual": actual})
    print(json.dumps({"verified": len(entries) - len(failures), "failures": failures}, ensure_ascii=False))
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
