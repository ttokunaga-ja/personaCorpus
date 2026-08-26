"""Synthetic closed-record validator for p03-full-008875; no I/O or network access."""

RECORD = {
  "record_id": "p03-full-008875",
  "source_id": "p03-src-008875",
  "record_date": "2026-07-15",
  "organization": "Cobalt Harbor Systems",
  "program": "Aegis Assurance",
  "control": "AC-27",
  "case_id": "CASE-260713-184",
  "evidence_set": "184",
  "risk_id": "RISK-031",
  "record_class": "evidence_archive_validation_tool_py_s008875.py",
  "final_disposition": "Closed; retained under synthetic GRC retention schedule",
  "residual_risk": "Medium",
  "retention_minimum_months": 18,
  "disposition_follow_up_anchors": [
    "2026-07-15"
  ],
  "ledger_traceability": [
    {
      "id": "org.cobalt-harbor",
      "canonical_value": "Cobalt Harbor Systems (CHS), a fictional managed logistics and harbor-operations platform."
    },
    {
      "id": "decision.dec-260713-004",
      "canonical_value": "Retain evidence set 184 under the synthetic GRC retention schedule; no live-source data is included."
    },
    {
      "id": "date.2026-07-15",
      "canonical_value": "Control owner validates the remediation plan and retention disposition."
    }
  ],
  "synthetic_notice": "Synthetic defensive governance record; no live-source data.",
  "review_focus": "retention indexing",
  "closure_finding": "the retention index links the closure decision",
  "closure_rationale": "the delayed attestation was documented and compensated",
  "conclusion": "Closed archive conclusion: the retention index links the closure decision; the delayed attestation was documented and compensated; focus: retention indexing."
}

def validate_closed_record(record: dict) -> list[str]:
    required = ("record_id", "case_id", "control", "risk_id", "final_disposition")
    return [key for key in required if not record.get(key)]

def summary() -> dict:
    return {"validation_id": "VAL-fe99827d6a82", "valid": not validate_closed_record(RECORD), "residual_risk": RECORD["residual_risk"]}
