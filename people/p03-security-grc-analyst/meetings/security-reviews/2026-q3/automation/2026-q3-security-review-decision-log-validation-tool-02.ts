// Cobalt Harbor Systems RISK-031 | GRC-REV-002974 | record DEC-260713-004 retention disposition with preparer-reviewer separation | ACT-260713-011 completion register | org.cobalt-harbor, decision.dec-260713-004 (DEC-260713-004), rel.case.timeline, period.2026-q3, term.no-kio-claim.
export type ReviewFoundation = { artifactId: string; control: string; evidenceSet: number; docket: string; };
export const reviewFoundation: ReviewFoundation = { artifactId: "p03-full-002974", control: "AC-27", evidenceSet: 184, docket: "GRC-REV-002974" };
export function isDefensiveRecord(record: ReviewFoundation): boolean { return record.control === "AC-27" && record.evidenceSet === 184; }
