"""Synthetic closed-record validator for p03-full-006003; no I/O or network access."""

RECORD = {
  "record_id": "p03-full-006003",
  "source_id": "p03-src-006003",
  "record_date": "2026-07-15",
  "organization": "Cobalt Harbor Systems",
  "program": "Aegis Assurance",
  "control": "AC-27",
  "case_id": "CASE-260713-184",
  "evidence_set": "184",
  "risk_id": "RISK-031",
  "record_class": "evidence_archive_validation_tool_py_s006003.py",
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
      "id": "case.case-260713-184",
      "canonical_value": "CASE-260713-184, the synthetic evidence intake case for evidence set 184; not a real incident."
    },
    {
      "id": "date.2026-07-15",
      "canonical_value": "Control owner validates the remediation plan and retention disposition."
    }
  ],
  "synthetic_notice": "Synthetic defensive governance record; no live-source data.",
  "review_focus": "retention indexing",
  "closure_finding": "the residual rating remains proportionate",
  "closure_rationale": "the case timeline was preserved without rewriting history",
  "conclusion": "Closed archive conclusion: the residual rating remains proportionate; the case timeline was preserved without rewriting history; focus: retention indexing."
}

def validate_closed_record(record: dict) -> list[str]:
    required = ("record_id", "case_id", "control", "risk_id", "final_disposition")
    return [key for key in required if not record.get(key)]

def summary() -> dict:
    return {"validation_id": "VAL-137d84f3a43b", "valid": not validate_closed_record(RECORD), "residual_risk": RECORD["residual_risk"]}
