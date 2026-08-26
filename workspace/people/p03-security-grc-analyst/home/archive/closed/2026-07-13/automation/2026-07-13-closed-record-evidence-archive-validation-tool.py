"""Synthetic closed-record validator for p03-full-004027; no I/O or network access."""

RECORD = {
  "record_id": "p03-full-004027",
  "source_id": "p03-src-004027",
  "record_date": "2026-07-13",
  "organization": "Cobalt Harbor Systems",
  "program": "Aegis Assurance",
  "control": "AC-27",
  "case_id": "CASE-260713-184",
  "evidence_set": "184",
  "risk_id": "RISK-031",
  "record_class": "evidence_archive_validation_tool_py_s004027.py",
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
  "review_focus": "retention indexing",
  "closure_finding": "the seven-day follow-up confirms completion",
  "closure_rationale": "the retention decision cites the approved synthetic schedule",
  "conclusion": "Closed archive conclusion: the seven-day follow-up confirms completion; the retention decision cites the approved synthetic schedule; focus: retention indexing."
}

def validate_closed_record(record: dict) -> list[str]:
    required = ("record_id", "case_id", "control", "risk_id", "final_disposition")
    return [key for key in required if not record.get(key)]

def summary() -> dict:
    return {"validation_id": "VAL-dd167ec8d3e7", "valid": not validate_closed_record(RECORD), "residual_risk": RECORD["residual_risk"]}
