/** Synthetic closed-record validation metadata; no I/O or network access. */
export const record = {
  "record_id": "p03-full-008274",
  "source_id": "p03-src-008274",
  "record_date": "2026-07-15",
  "organization": "Cobalt Harbor Systems",
  "program": "Aegis Assurance",
  "control": "AC-27",
  "case_id": "CASE-260713-184",
  "evidence_set": "184",
  "risk_id": "RISK-031",
  "record_class": "final_disposition_validation_tool_ts_s008274.ts",
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
  "review_focus": "approval separation",
  "closure_finding": "the closure packet contains no restricted data",
  "closure_rationale": "all 24 required synthetic items reconciled",
  "conclusion": "Closed archive conclusion: the closure packet contains no restricted data; all 24 required synthetic items reconciled; focus: approval separation."
} as const;
export function validateClosedRecord(value: typeof record): string[] {
  return ["record_id", "case_id", "control", "risk_id", "final_disposition"].filter((key) => !value[key as keyof typeof value]);
}
export const validationId = "VAL-c4e7eb4db89a";
