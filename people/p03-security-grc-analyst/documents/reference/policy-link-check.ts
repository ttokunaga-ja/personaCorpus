/** Validate local policy-reference records without fetching any URL. */
type PolicyLink = { controlId: string; policyId: string; reviewDate: string };

export function validateLinks(rows: PolicyLink[]): string[] {
  const issues: string[] = [];
  const ids = new Set<string>();
  for (const row of rows) {
    if (!/^AC-[0-9]{2}$/.test(row.controlId)) issues.push(`invalid control: ${row.controlId}`);
    if (!/^POL-[A-Z0-9-]+$/.test(row.policyId)) issues.push(`invalid policy: ${row.policyId}`);
    if (!/^2026-[0-9]{2}-[0-9]{2}$/.test(row.reviewDate)) issues.push(`invalid review date: ${row.reviewDate}`);
    const key = `${row.controlId}/${row.policyId}`;
    if (ids.has(key)) issues.push(`duplicate link: ${key}`);
    ids.add(key);
  }
  return issues;
}

const sample: PolicyLink[] = [
  { controlId: "AC-27", policyId: "POL-EVIDENCE-01", reviewDate: "2026-07-13" },
];
console.log(JSON.stringify({ checked: sample.length, issues: validateLinks(sample) }, null, 2));
