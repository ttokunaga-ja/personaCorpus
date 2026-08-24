#!/usr/bin/env python3
"""Validate the synthetic ForgeLine archived NCR evidence store.

This checker has no network or credential behavior.  It confirms the fixed
closure anchors for Product Alpha and reports a compact local integrity result.
"""

from __future__ import annotations

import argparse
import hashlib
import sqlite3
from pathlib import Path


EXPECTED = {
    "product": "Product Alpha",
    "closure_date": "2026-08-11",
    "ncr_id": "NCR-026",
    "tolerance_mm": 0.18,
    "disposition": "closed",
}


def verify(database_path: Path) -> tuple[bool, str]:
    if not database_path.is_file():
        return False, f"missing evidence store: {database_path}"
    with sqlite3.connect(f"file:{database_path}?mode=ro", uri=True) as connection:
        quick_check = connection.execute("PRAGMA quick_check").fetchone()[0]
        if quick_check != "ok":
            return False, f"sqlite quick_check failed: {quick_check}"
        row = connection.execute(
            "SELECT product, closure_date, ncr_id, tolerance_mm, disposition, source_digest "
            "FROM closed_ncr_evidence WHERE evidence_id = ?",
            ("EVD-NCR-026-20260811",),
        ).fetchone()
        if row is None:
            return False, "required synthetic evidence row is absent"
        values = dict(zip([*EXPECTED, "source_digest"], row, strict=True))
        for field, expected in EXPECTED.items():
            if values[field] != expected:
                return False, f"anchor mismatch for {field}: {values[field]!r}"
        digest_input = "ForgeLine|Product Alpha|2026-08-11|NCR-026|+-0.18 mm".encode()
        if values["source_digest"] != hashlib.sha256(digest_input).hexdigest():
            return False, "source digest mismatch"
        checks = connection.execute(
            "SELECT COUNT(*) FROM closure_checks WHERE evidence_id = ?",
            ("EVD-NCR-026-20260811",),
        ).fetchone()[0]
        if checks != 3:
            return False, f"expected three closure checks, found {checks}"
    return True, "archive integrity verified: ForgeLine / Product Alpha / NCR-026 / 2026-08-11 / ±0.18 mm"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "database",
        nargs="?",
        type=Path,
        default=Path(__file__).with_name("closed-ncr-evidence-store.sqlite"),
    )
    arguments = parser.parse_args()
    valid, message = verify(arguments.database)
    print(message)
    return 0 if valid else 1


if __name__ == "__main__":
    raise SystemExit(main())
