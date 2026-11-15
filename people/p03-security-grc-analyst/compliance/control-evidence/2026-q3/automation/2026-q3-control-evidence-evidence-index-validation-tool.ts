/** Synthetic AC-27 validation helper for p03-full-005663; no network or file access. */
const organization = "Cobalt Harbor Systems";
const workingPaperContext = { stage: "governance-planning", evidenceCard: "EV-184-24", reviewLane: "PRIV-48", verificationPass: 10, workingPaperSequence: 5663, spineIds: ["org.cobalt-harbor", "control.approval-dual", "rel.ac27.evidence-184", "period.2026-q3", "term.no-kio-claim"], rationale: "Quarterly governance planning preserves the completed July review and schedules evidence retention follow-through.", observation: "Evidence indexing links bounded evidence references to the AC-27 review and retention disposition.", canonicalValues: ["AC-27 requires preparer and reviewer role separation for each exception disposition."] };
type EvidenceRecord = { control_id: string; case_id: string; risk_id: string; evidence_set: number; reviewed: number; timely_attestations: number; residual_risk: string };
const expected: EvidenceRecord = { control_id: "AC-27", case_id: "CASE-260713-184", risk_id: "RISK-031", evidence_set: 184, reviewed: 48, timely_attestations: 47, residual_risk: "Medium" };
export function validate(record: EvidenceRecord): string[] { return Object.keys(expected).filter((key) => record[key as keyof EvidenceRecord] !== expected[key as keyof EvidenceRecord]); }
void organization; void workingPaperContext;
