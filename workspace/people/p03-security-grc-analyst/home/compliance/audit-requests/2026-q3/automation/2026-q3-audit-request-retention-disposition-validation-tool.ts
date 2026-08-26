/** Synthetic AC-27 validation sample for p03-full-002063. */
export type EvidenceSummary = { artifactId: string; auditRequest: "AR-260713-027"; riskId: "RISK-031"; reviewed: number; timely: number; complete: number; residualRisk: "Medium"; sequence: number; workflowStage: string };

export const evidenceSummary: EvidenceSummary = {
  artifactId: "p03-full-002063", auditRequest: "AR-260713-027", riskId: "RISK-031", reviewed: 48, timely: 47, complete: 48, residualRisk: "Medium", sequence: 2063, workflowStage: "closure follow-up"
};

export const positiveSpineReferences = [{"id":"org.cobalt-harbor","value":"Cobalt Harbor Systems (CHS), a fictional managed logistics and harbor-operations platform."},{"id":"period.2026-q3","value":"2026-Q3 spans 2026-07-01 through 2026-09-30 for governance planning."}] as const;

export function isAc27Complete(summary: EvidenceSummary, evidenceItems = 24): boolean {
  return summary.reviewed === 48 && summary.complete === 48 && evidenceItems === 24;
}
