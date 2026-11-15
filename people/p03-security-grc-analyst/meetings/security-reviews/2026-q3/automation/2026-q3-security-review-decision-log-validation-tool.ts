// Cobalt Harbor Systems RISK-031 | GRC-REV-002934 | record DEC-260713-004 retention disposition with preparer-reviewer separation | AR-260713-027 response packet | org.cobalt-harbor, period.2026-q3, rel.case.timeline, term.no-kio-claim.
export type ReviewFoundation = { artifactId: string; control: string; evidenceSet: number; docket: string; };
export const reviewFoundation: ReviewFoundation = { artifactId: "p03-full-002934", control: "AC-27", evidenceSet: 184, docket: "GRC-REV-002934" };
export function isDefensiveRecord(record: ReviewFoundation): boolean { return record.control === "AC-27" && record.evidenceSet === 184; }
