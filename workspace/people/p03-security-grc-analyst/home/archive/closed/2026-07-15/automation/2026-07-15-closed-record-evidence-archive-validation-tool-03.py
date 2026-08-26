"""Synthetic closed-record validator for p03-full-009575; no I/O or network access."""

RECORD = {
  "record_id": "p03-full-009575",
  "source_id": "p03-src-009575",
  "record_date": "2026-07-15",
  "organization": "Cobalt Harbor Systems",
  "program": "Aegis Assurance",
  "control": "AC-27",
  "case_id": "CASE-260713-184",
  "evidence_set": "184",
  "risk_id": "RISK-031",
  "record_class": "evidence_archive_validation_tool_py_s009575.py",
  "final_disposition": "Closed; retained under synthetic GRC retention schedule",
  "residual_risk": "Medium",
  "retention_minimum_months": 18,
  "disposition_follow_up_anchors": [
    "2026-07-20",
    "2026-07-15"
  ],
  "ledger_traceability": [
    {
      "id": "org.cobalt-harbor",
      "canonical_value": "Cobalt Harbor Systems (CHS), a fictional managed logistics and harbor-operations platform."
    },
    {
      "id": "date.2026-07-20",
      "canonical_value": "Seven-day follow-up validates closure evidence and residual-risk status."
    },
    {
      "id": "date.2026-07-15",
      "canonical_value": "Control owner validates the remediation plan and retention disposition."
    }
  ],
  "synthetic_notice": "Synthetic defensive governance record; no live-source data.",
  "review_focus": "follow-up confirmation",
  "closure_finding": "the control conclusion is supported by synthetic records",
  "closure_rationale": "the case timeline was preserved without rewriting history",
  "conclusion": "Closed archive conclusion: the control conclusion is supported by synthetic records; the case timeline was preserved without rewriting history; focus: follow-up confirmation."
}

def validate_closed_record(record: dict) -> list[str]:
    required = ("record_id", "case_id", "control", "risk_id", "final_disposition")
    return [key for key in required if not record.get(key)]

def summary() -> dict:
    return {"validation_id": "VAL-b0c00278bb50", "valid": not validate_closed_record(RECORD), "residual_risk": RECORD["residual_risk"]}
