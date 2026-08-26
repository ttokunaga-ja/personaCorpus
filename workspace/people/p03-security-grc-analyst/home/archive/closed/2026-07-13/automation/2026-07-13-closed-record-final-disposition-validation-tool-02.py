"""Synthetic closed-record validator for p03-full-007774; no I/O or network access."""

RECORD = {
  "record_id": "p03-full-007774",
  "source_id": "p03-src-007774",
  "record_date": "2026-07-13",
  "organization": "Cobalt Harbor Systems",
  "program": "Aegis Assurance",
  "control": "AC-27",
  "case_id": "CASE-260713-184",
  "evidence_set": "184",
  "risk_id": "RISK-031",
  "record_class": "final_disposition_validation_tool_py_s007774.py",
  "final_disposition": "Closed; retained under synthetic GRC retention schedule",
  "residual_risk": "Medium",
  "retention_minimum_months": 18,
  "disposition_follow_up_anchors": [
    "2026-07-13"
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
      "id": "date.2026-07-13",
      "canonical_value": "Primary review date: evidence intake opens 08:30 UTC and the AC-27 review closes 16:45 UTC."
    }
  ],
  "synthetic_notice": "Synthetic defensive governance record; no live-source data.",
  "review_focus": "exception closure",
  "closure_finding": "the final disposition preserves the original exception",
  "closure_rationale": "all 24 required synthetic items reconciled",
  "conclusion": "Closed archive conclusion: the final disposition preserves the original exception; all 24 required synthetic items reconciled; focus: exception closure."
}

def validate_closed_record(record: dict) -> list[str]:
    required = ("record_id", "case_id", "control", "risk_id", "final_disposition")
    return [key for key in required if not record.get(key)]

def summary() -> dict:
    return {"validation_id": "VAL-246fa960e359", "valid": not validate_closed_record(RECORD), "residual_risk": RECORD["residual_risk"]}
