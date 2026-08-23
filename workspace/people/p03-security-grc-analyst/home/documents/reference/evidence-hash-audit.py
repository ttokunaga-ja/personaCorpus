#!/usr/bin/env python3
"""Create a local SHA-256 evidence inventory; no network or credential access."""
from __future__ import annotations

import argparse
import hashlib
from pathlib import Path


def digest(path: Path) -> str:
    hasher = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(65536), b""):
            hasher.update(block)
    return hasher.hexdigest()


def main() -> int:
    parser = argparse.ArgumentParser(description="Audit a local evidence directory")
    parser.add_argument("directory", type=Path)
    args = parser.parse_args()
    directory = args.directory.resolve()
    if not directory.is_dir():
        parser.error("directory must exist")
    for path in sorted(p for p in directory.rglob("*") if p.is_file()):
        print(f"{digest(path)}  {path.relative_to(directory)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
