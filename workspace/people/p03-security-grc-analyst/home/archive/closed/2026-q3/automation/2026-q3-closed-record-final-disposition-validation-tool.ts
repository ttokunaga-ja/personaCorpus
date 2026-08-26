/** Synthetic closed-record validation metadata; no I/O or network access. */
export const record = {
  "record_id": "p03-full-007974",
  "source_id": "p03-src-007974",
  "record_date": "2026-Q3",
  "organization": "Cobalt Harbor Systems",
  "program": "Aegis Assurance",
  "control": "AC-27",
  "case_id": "CASE-260713-184",
  "evidence_set": "184",
  "risk_id": "RISK-031",
  "record_class": "final_disposition_validation_tool_ts_s007974.ts",
  "final_disposition": "Closed; retained under synthetic GRC retention schedule",
  "residual_risk": "Medium",
  "retention_minimum_months": 18,
  "disposition_follow_up_anchors": [],
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
      "id": "period.2026-q3",
      "canonical_value": "Governance planning period from 2026-07-01 through 2026-09-30."
    }
  ],
  "synthetic_notice": "Synthetic defensive governance record; no live-source data.",
  "review_focus": "exception closure",
  "closure_finding": "the control conclusion is supported by synthetic records",
  "closure_rationale": "the closure review used minimized category-level evidence",
  "conclusion": "Closed archive conclusion: the control conclusion is supported by synthetic records; the closure review used minimized category-level evidence; focus: exception closure."
} as const;
export function validateClosedRecord(value: typeof record): string[] {
  return ["record_id", "case_id", "control", "risk_id", "final_disposition"].filter((key) => !value[key as keyof typeof value]);
}
export const validationId = "VAL-356b62b30047";
