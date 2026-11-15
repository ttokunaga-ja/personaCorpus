/** Synthetic closed-record validation metadata; no I/O or network access. */
export const record = {
  "record_id": "p03-full-003962",
  "source_id": "p03-src-003962",
  "record_date": "2026-07-14",
  "organization": "Cobalt Harbor Systems",
  "program": "Aegis Assurance",
  "control": "AC-27",
  "case_id": "CASE-260713-184",
  "evidence_set": "184",
  "risk_id": "RISK-031",
  "record_class": "final_disposition_validation_tool_ts_s003962.ts",
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
  "review_focus": "approval separation",
  "closure_finding": "the seven-day follow-up confirms completion",
  "closure_rationale": "the case timeline was preserved without rewriting history",
  "conclusion": "Closed archive conclusion: the seven-day follow-up confirms completion; the case timeline was preserved without rewriting history; focus: approval separation."
} as const;
export function validateClosedRecord(value: typeof record): string[] {
  return ["record_id", "case_id", "control", "risk_id", "final_disposition"].filter((key) => !value[key as keyof typeof value]);
}
export const validationId = "VAL-0ae6df9278d6";
