"""Synthetic defensive validation helper for p03-full-003840; no network or filesystem access.
Assigned team-shared topic: handoff; record focus: defensive validation; locator: 2026-07-20-team-assurance-handoff-validation-tool-py;assigned_source_slot=003840;ledger_spine_refs=org.cobalt-harbor=Cobalt Harbor Systems | team.grc=Security GRC owns the AC-27 evidence register risk treatment tracking and audit-request coordination | date.2026-07-20=Seven-day follow-up validates closure evidence and residual-risk status;spine_dates=2026-07-13;2026-07-14;2026-07-15;2026-07-20.
"""
ARTIFACT_ID = "p03-full-003840"
CONTROL_ID = "AC-27"
RECORD_FOCUS = "defensive validation"

def validate_review(record: dict) -> list[str]:
    """Return missing governance fields for this synthetic handoff record."""
    required = ("evidence_set", "reviewed", "residual_risk", "record_focus")
    return [field for field in required if field not in record]

if __name__ == "__main__":
    print({"artifact_id": ARTIFACT_ID, "missing": validate_review({"evidence_set": 184, "reviewed": 48, "residual_risk": "Medium", "record_focus": RECORD_FOCUS})})
