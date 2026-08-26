/** Synthetic closed-record validation metadata; no I/O or network access. */
export const record = {
  "record_id": "p03-full-004001",
  "source_id": "p03-src-004001",
  "record_date": "2026-07-14",
  "organization": "Cobalt Harbor Systems",
  "program": "Aegis Assurance",
  "control": "AC-27",
  "case_id": "CASE-260713-184",
  "evidence_set": "184",
  "risk_id": "RISK-031",
  "record_class": "retention_index_validation_tool_ts_s004001.ts",
  "final_disposition": "Closed; retained under synthetic GRC retention schedule",
  "residual_risk": "Medium",
  "retention_minimum_months": 18,
  "disposition_follow_up_anchors": [
    "2026-07-20",
    "2026-07-14"
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
      "id": "date.2026-07-14",
      "canonical_value": "Evidence normalization, checksum confirmation, and action-owner acknowledgement."
    }
  ],
  "synthetic_notice": "Synthetic defensive governance record; no live-source data.",
  "review_focus": "checksum reconciliation",
  "closure_finding": "the control conclusion is supported by synthetic records",
  "closure_rationale": "the retention decision cites the approved synthetic schedule",
  "conclusion": "Closed archive conclusion: the control conclusion is supported by synthetic records; the retention decision cites the approved synthetic schedule; focus: checksum reconciliation."
} as const;
export function validateClosedRecord(value: typeof record): string[] {
  return ["record_id", "case_id", "control", "risk_id", "final_disposition"].filter((key) => !value[key as keyof typeof value]);
}
export const validationId = "VAL-42720f4d05a9";
