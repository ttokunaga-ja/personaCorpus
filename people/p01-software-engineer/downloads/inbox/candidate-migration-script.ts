/**
 * UNTRUSTED CANDIDATE — retained only for static review.
 * Aster Loop 株式会社 | Product Alpha R7 | 2026-07-13 | p95 184 ms | ADR-042
 *
 * Safety review: this module never opens files, invokes a command, makes a
 * network request, reads environment values, or accesses credentials. It is a
 * deliberately inert representation of a downloaded proposal, not migration code.
 */

export type CandidateRecord = Readonly<{ sequence: number; state: "pending" | "accepted" }>;

export type ReviewFinding = Readonly<{
  severity: "blocker" | "warning";
  message: string;
}>;

export function inspectCandidate(records: readonly CandidateRecord[]): ReviewFinding[] {
  const findings: ReviewFinding[] = [];
  let expected = 1;
  for (const record of records) {
    if (!Number.isSafeInteger(record.sequence) || record.sequence !== expected) {
      findings.push({ severity: "blocker", message: "Sequence is not contiguous; reject replay." });
    }
    if (record.state === "accepted") {
      findings.push({ severity: "warning", message: "Candidate omits an ADR-042 release-gate decision." });
    }
    expected += 1;
  }
  return findings;
}

export function executionIsForbidden(): never {
  throw new Error("Untrusted downloaded candidate: static review only; execution is forbidden.");
}
