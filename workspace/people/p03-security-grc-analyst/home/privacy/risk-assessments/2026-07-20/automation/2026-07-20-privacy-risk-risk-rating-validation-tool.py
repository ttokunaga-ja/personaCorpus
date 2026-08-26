# Assessment record ordinal 8213: Record 8213 concludes that risk rating requires a bounded AC-27 evidence review for 2026-07-20; RISK-031 remains Medium after the compensating review, with no personal records in scope.
# Positive spine traceability: org.cobalt-harbor = Cobalt Harbor Systems | measure.residual-rating = RISK-031 residual rating is Medium after compensating review and follow-up actions | date.2026-07-20 = seven-day follow-up validates closure evidence and residual-risk status
"""Safe validation fixture for p03-src-008213; no network or file access."""
from __future__ import annotations

FOUNDATION = {"source_id": "p03-src-008213", "control": "AC-27", "risk": "RISK-031"}

def validate(record: dict) -> list[str]:
    required = ("source_id", "control", "risk")
    return [key for key in required if not record.get(key)]

if __name__ == "__main__":
    raise SystemExit(0 if not validate(FOUNDATION) else 1)
