"""Synthetic defensive validation helper for p03-full-004657; no network or filesystem access.
Assigned team-shared topic: shared-register; record focus: defensive validation; locator: 2026-07-13-team-assurance-shared-register-validation-tool-py;assigned_source_slot=004657;ledger_spine_refs=org.cobalt-harbor=Cobalt Harbor Systems | team.soc=Security Operations owns bounded SIEM exports detection review evidence and incident evidence-chain handoff | date.2026-07-13=Primary review date; evidence intake opens 08:30 UTC; AC-27 review closes 16:45 UTC;spine_dates=2026-07-13;2026-07-14;2026-07-15;2026-07-20.
"""
ARTIFACT_ID = "p03-full-004657"
CONTROL_ID = "AC-27"
RECORD_FOCUS = "defensive validation"

def validate_review(record: dict) -> list[str]:
    """Return missing governance fields for this synthetic shared-register record."""
    required = ("evidence_set", "reviewed", "residual_risk", "record_focus")
    return [field for field in required if field not in record]

if __name__ == "__main__":
    print({"artifact_id": ARTIFACT_ID, "missing": validate_review({"evidence_set": 184, "reviewed": 48, "residual_risk": "Medium", "record_focus": RECORD_FOCUS})})
