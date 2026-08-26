/** Synthetic closed-record validation metadata; no I/O or network access. */
export const record = {
  "record_id": "p03-full-004035",
  "source_id": "p03-src-004035",
  "record_date": "2026-07-14",
  "organization": "Cobalt Harbor Systems",
  "program": "Aegis Assurance",
  "control": "AC-27",
  "case_id": "CASE-260713-184",
  "evidence_set": "184",
  "risk_id": "RISK-031",
  "record_class": "evidence_archive_validation_tool_ts_s004035.ts",
  "final_disposition": "Closed; retained under synthetic GRC retention schedule",
  "residual_risk": "Medium",
  "retention_minimum_months": 18,
  "disposition_follow_up_anchors": [
    "2026-07-14"
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
      "id": "date.2026-07-14",
      "canonical_value": "Evidence normalization, checksum confirmation, and action-owner acknowledgement."
    }
  ],
  "synthetic_notice": "Synthetic defensive governance record; no live-source data.",
  "review_focus": "retention indexing",
  "closure_finding": "the bounded evidence set is complete",
  "closure_rationale": "the follow-up confirms the remediation register",
  "conclusion": "Closed archive conclusion: the bounded evidence set is complete; the follow-up confirms the remediation register; focus: retention indexing."
} as const;
export function validateClosedRecord(value: typeof record): string[] {
  return ["record_id", "case_id", "control", "risk_id", "final_disposition"].filter((key) => !value[key as keyof typeof value]);
}
export const validationId = "VAL-72a2e06a6e44";
