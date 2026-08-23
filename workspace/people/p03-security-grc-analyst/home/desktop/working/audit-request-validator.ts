/** Validate a synthetic audit request before it enters a local review queue. */
type AuditRequest = { requestId: string; control: string; dueDate: string; evidenceIds: string[] };

function validate(request: AuditRequest): string[] {
  const issues: string[] = [];
  if (!/^AR-\d{8}-[A-Z0-9-]+$/.test(request.requestId)) issues.push("requestId format is invalid");
  if (!/^\d{4}-\d{2}-\d{2}$/.test(request.dueDate)) issues.push("dueDate must be ISO-8601");
  if (request.evidenceIds.length === 0) issues.push("at least one evidence reference is required");
  if (!request.control.includes("AC-")) issues.push("control must include an AC identifier");
  return issues;
}

const sample: AuditRequest = { requestId: "AR-20260713-AC27", control: "Aegis Control AC-27", dueDate: "2026-07-13", evidenceIds: ["evidence set 184"] };
console.log(JSON.stringify({ valid: validate(sample).length === 0, issues: validate(sample) }));
