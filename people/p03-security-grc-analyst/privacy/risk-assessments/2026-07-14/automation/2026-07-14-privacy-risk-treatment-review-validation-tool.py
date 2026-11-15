# Assessment record ordinal 2811: Record 2811 concludes that treatment review requires a bounded AC-27 evidence review for 2026-07-14; RISK-031 remains Medium after the compensating review, with no personal records in scope.
# Positive spine traceability: org.cobalt-harbor = Cobalt Harbor Systems | team.privacy = Privacy Assurance owns privacy-risk assessments and data-use reviews | date.2026-07-14 = evidence normalization, checksum confirmation, and action-owner acknowledgement
"""Safe validation fixture for p03-src-002811; no network or file access."""
from __future__ import annotations

FOUNDATION = {"source_id": "p03-src-002811", "control": "AC-27", "risk": "RISK-031"}

def validate(record: dict) -> list[str]:
    required = ("source_id", "control", "risk")
    return [key for key in required if not record.get(key)]

if __name__ == "__main__":
    raise SystemExit(0 if not validate(FOUNDATION) else 1)
