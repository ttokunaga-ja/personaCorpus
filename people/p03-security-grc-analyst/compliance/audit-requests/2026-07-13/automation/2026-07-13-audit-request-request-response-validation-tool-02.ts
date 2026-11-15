/** Synthetic AC-27 validation sample for p03-full-002100. */
export type EvidenceSummary = { artifactId: string; auditRequest: "AR-260713-027"; riskId: "RISK-031"; reviewed: number; timely: number; complete: number; residualRisk: "Medium"; sequence: number; workflowStage: string };

export const evidenceSummary: EvidenceSummary = {
  artifactId: "p03-full-002100", auditRequest: "AR-260713-027", riskId: "RISK-031", reviewed: 48, timely: 47, complete: 48, residualRisk: "Medium", sequence: 2100, workflowStage: "intake triage"
};

export const positiveSpineReferences = [{"id":"org.cobalt-harbor","value":"Cobalt Harbor Systems (CHS), a fictional managed logistics and harbor-operations platform."},{"id":"audit.ar-260713-027","value":"AR-260713-027 is the fictional AC-27 audit request for bounded review material."},{"id":"date.2026-07-13","value":"2026-07-13 is the primary review date; intake opens 08:30 UTC and AC-27 review closes 16:45 UTC."}] as const;

export function isAc27Complete(summary: EvidenceSummary, evidenceItems = 24): boolean {
  return summary.reviewed === 48 && summary.complete === 48 && evidenceItems === 24;
}
