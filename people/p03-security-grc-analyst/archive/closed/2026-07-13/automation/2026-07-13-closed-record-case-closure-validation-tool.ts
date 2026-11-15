/** Synthetic closed-record validation metadata; no I/O or network access. */
export const record = {
  "record_id": "p03-full-006160",
  "source_id": "p03-src-006160",
  "record_date": "2026-07-13",
  "organization": "Cobalt Harbor Systems",
  "program": "Aegis Assurance",
  "control": "AC-27",
  "case_id": "CASE-260713-184",
  "evidence_set": "184",
  "risk_id": "RISK-031",
  "record_class": "case_closure_validation_tool_ts_s006160.ts",
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
  "review_focus": "attestation timeliness",
  "closure_finding": "the closure packet contains no restricted data",
  "closure_rationale": "reviewer and preparer roles remained separate",
  "conclusion": "Closed archive conclusion: the closure packet contains no restricted data; reviewer and preparer roles remained separate; focus: attestation timeliness."
} as const;
export function validateClosedRecord(value: typeof record): string[] {
  return ["record_id", "case_id", "control", "risk_id", "final_disposition"].filter((key) => !value[key as keyof typeof value]);
}
export const validationId = "VAL-a910db0f1efe";
