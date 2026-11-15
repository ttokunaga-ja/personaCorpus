"""Synthetic defensive validation helper for p03-full-008801; no network or filesystem access.
Assigned team-shared topic: shared-register; record focus: defensive validation; locator: 2026-07-15-team-assurance-shared-register-validation-tool-py;assigned_source_slot=008801;ledger_spine_refs=org.cobalt-harbor=Cobalt Harbor Systems | team.privacy=Privacy Assurance owns privacy-risk assessments and data-use reviews | date.2026-07-15=Control owner validates remediation plan and retention disposition;spine_dates=2026-07-13;2026-07-14;2026-07-15;2026-07-20.
"""
ARTIFACT_ID = "p03-full-008801"
CONTROL_ID = "AC-27"
RECORD_FOCUS = "defensive validation"

def validate_review(record: dict) -> list[str]:
    """Return missing governance fields for this synthetic shared-register record."""
    required = ("evidence_set", "reviewed", "residual_risk", "record_focus")
    return [field for field in required if field not in record]

if __name__ == "__main__":
    print({"artifact_id": ARTIFACT_ID, "missing": validate_review({"evidence_set": 184, "reviewed": 48, "residual_risk": "Medium", "record_focus": RECORD_FOCUS})})
