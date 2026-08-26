/** Synthetic AC-27 validation sample for p03-full-005245. */
export type EvidenceSummary = { artifactId: string; auditRequest: "AR-260713-027"; riskId: "RISK-031"; reviewed: number; timely: number; complete: number; residualRisk: "Medium"; sequence: number; workflowStage: string };

export const evidenceSummary: EvidenceSummary = {
  artifactId: "p03-full-005245", auditRequest: "AR-260713-027", riskId: "RISK-031", reviewed: 48, timely: 47, complete: 48, residualRisk: "Medium", sequence: 5245, workflowStage: "request clarification"
};

export const positiveSpineReferences = [{"id":"org.cobalt-harbor","value":"Cobalt Harbor Systems (CHS), a fictional managed logistics and harbor-operations platform."},{"id":"evidence.set-184","value":"Evidence set 184 contains 24 synthetic items: 12 attestations, 6 approvals, 4 checksum manifests, and 2 exception records."},{"id":"period.2026-q3","value":"2026-Q3 spans 2026-07-01 through 2026-09-30 for governance planning."}] as const;

export function isAc27Complete(summary: EvidenceSummary, evidenceItems = 24): boolean {
  return summary.reviewed === 48 && summary.complete === 48 && evidenceItems === 24;
}
