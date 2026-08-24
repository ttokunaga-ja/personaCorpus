#!/usr/bin/env python3
"""Validate synthetic ForgeLine quality-review attachments before triage.

This small, offline utility is intentionally limited to a reviewed allowlist.
It records no credentials and makes no network calls.
"""

from __future__ import annotations

import argparse
import hashlib
from dataclasses import dataclass
from pathlib import Path


ALLOWED_SUFFIXES = {".csv", ".pdf", ".xlsx", ".png", ".txt", ".tsv"}
MAX_BYTES = 25 * 1024 * 1024


@dataclass(frozen=True)
class AttachmentCheck:
    name: str
    bytes: int
    sha256: str
    accepted: bool
    reason: str


def inspect_attachment(path: Path) -> AttachmentCheck:
    if not path.is_file():
        return AttachmentCheck(path.name, 0, "", False, "not a regular file")
    size = path.stat().st_size
    if size == 0:
        return AttachmentCheck(path.name, size, "", False, "empty file")
    if size > MAX_BYTES:
        return AttachmentCheck(path.name, size, "", False, "exceeds 25 MiB review limit")
    if path.suffix.lower() not in ALLOWED_SUFFIXES:
        return AttachmentCheck(path.name, size, "", False, "suffix is not in review allowlist")
    digest = hashlib.sha256(path.read_bytes()).hexdigest()
    return AttachmentCheck(path.name, size, digest, True, "ready for NCR-026 review")


def main() -> int:
    parser = argparse.ArgumentParser(description="ForgeLine attachment precheck")
    parser.add_argument("attachment", type=Path)
    args = parser.parse_args()
    result = inspect_attachment(args.attachment)
    print(f"{result.name}: {result.reason}; bytes={result.bytes}; sha256={result.sha256}")
    return 0 if result.accepted else 2


if __name__ == "__main__":
    raise SystemExit(main())
