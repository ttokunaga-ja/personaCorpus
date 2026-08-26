"""Synthetic closed-record validator for p03-full-003990; no I/O or network access."""

RECORD = {
  "record_id": "p03-full-003990",
  "source_id": "p03-src-003990",
  "record_date": "2026-07-13",
  "organization": "Cobalt Harbor Systems",
  "program": "Aegis Assurance",
  "control": "AC-27",
  "case_id": "CASE-260713-184",
  "evidence_set": "184",
  "risk_id": "RISK-031",
  "record_class": "final_disposition_validation_tool_py_s003990.py",
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
      "id": "case.case-260713-184",
      "canonical_value": "CASE-260713-184, the synthetic evidence intake case for evidence set 184; not a real incident."
    },
    {
      "id": "date.2026-07-13",
      "canonical_value": "Primary review date: evidence intake opens 08:30 UTC and the AC-27 review closes 16:45 UTC."
    }
  ],
  "synthetic_notice": "Synthetic defensive governance record; no live-source data.",
  "review_focus": "exception closure",
  "closure_finding": "the closure packet contains no restricted data",
  "closure_rationale": "the retention decision cites the approved synthetic schedule",
  "conclusion": "Closed archive conclusion: the closure packet contains no restricted data; the retention decision cites the approved synthetic schedule; focus: exception closure."
}

def validate_closed_record(record: dict) -> list[str]:
    required = ("record_id", "case_id", "control", "risk_id", "final_disposition")
    return [key for key in required if not record.get(key)]

def summary() -> dict:
    return {"validation_id": "VAL-765dc251f585", "valid": not validate_closed_record(RECORD), "residual_risk": RECORD["residual_risk"]}
