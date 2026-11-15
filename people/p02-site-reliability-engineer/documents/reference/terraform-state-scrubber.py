#!/usr/bin/env python3
"""Emit a redacted Terraform state copy without changing the source file."""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Any


SENSITIVE_KEY = re.compile(r"(?:password|secret|token|api[_-]?key|private[_-]?key|credential)", re.I)


def scrub(value: Any, key: str = "") -> Any:
    """Recursively redact sensitive keys while preserving the state structure."""
    if SENSITIVE_KEY.search(key):
        return "[redacted]"
    if isinstance(value, dict):
        return {item_key: scrub(item_value, item_key) for item_key, item_value in value.items()}
    if isinstance(value, list):
        return [scrub(item) for item in value]
    return value


def main() -> None:
    parser = argparse.ArgumentParser(description="Print a redacted Terraform state JSON document.")
    parser.add_argument("state", type=Path, help="local Terraform state JSON to read")
    args = parser.parse_args()
    if not args.state.is_file():
        parser.error("state must be a readable regular file")
    try:
        state = json.loads(args.state.read_text(encoding="utf-8"))
    except json.JSONDecodeError as error:
        parser.error(f"state is not valid JSON: {error.msg}")
    json.dump(scrub(state), sys.stdout, indent=2, sort_keys=True)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
