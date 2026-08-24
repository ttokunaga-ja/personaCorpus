#!/usr/bin/env python3
"""Offline filename/content redactor for HelioDesk training attachments."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

EMAIL_RE = re.compile(r"(?i)\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}\b")
TOKEN_RE = re.compile(
    r"(?i)\b(?:api[_-]?key|token|secret|bearer)[\s:=]+[A-Z0-9_./+=-]{8,}\b"
)
SAFE_NAME_RE = re.compile(r"[^A-Za-z0-9._-]+")


def redact(value: str) -> str:
    """Replace email addresses and common token assignments with neutral markers."""
    value = EMAIL_RE.sub("[REDACTED EMAIL]", value)
    return TOKEN_RE.sub("[REDACTED TOKEN]", value)


def safe_name(name: str) -> str:
    """Return a path-free, filesystem-safe filename without altering the source file."""
    basename = Path(name).name
    cleaned = SAFE_NAME_RE.sub("_", basename).strip("._")
    return cleaned or "attachment"


def main() -> int:
    parser = argparse.ArgumentParser(description="Inspect and redact one UTF-8 attachment offline.")
    parser.add_argument("attachment", type=Path, help="local UTF-8 text attachment to inspect")
    args = parser.parse_args()

    path = args.attachment
    if not path.is_file():
        parser.error("attachment must be an existing regular file")
    if path.stat().st_size > 2_000_000:
        parser.error("attachment exceeds the 2 MB offline inspection limit")

    try:
        content = path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        parser.error("attachment is not UTF-8 text; inspect binary files with a dedicated tool")

    print(f"sanitized_filename={safe_name(path.name)}")
    print("redacted_content:")
    print(redact(content), end="" if content.endswith("\n") else "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
