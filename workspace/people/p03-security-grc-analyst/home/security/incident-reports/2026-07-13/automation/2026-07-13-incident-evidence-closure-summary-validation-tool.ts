// Cobalt Harbor Systems defensive, synthetic validation record IR-001123 for RISK-031.
// Content spine references: org.cobalt-harbor = Cobalt Harbor Systems, a fictional managed logistics and harbor-operations platform; evidence.ev-184-07 = EV-184-07, the Sentinel Export minimization statement reporting only synthetic event categories and counts; date.2026-07-13 = 2026-07-13, primary review date; evidence intake 08:30 UTC and AC-27 review close 16:45 UTC
const expected = { caseId: "CASE-260713-184", control: "AC-27", risk: "RISK-031", completed: 48 };
export function validateEvidence(record: Record<string, unknown>): string[] {
  const issues: string[] = [];
  if (record.caseId !== expected.caseId) issues.push("case_id");
  if (record.control !== expected.control) issues.push("control");
  if (record.risk !== expected.risk || record.completed !== expected.completed) issues.push("assessment");
  return issues;
}
