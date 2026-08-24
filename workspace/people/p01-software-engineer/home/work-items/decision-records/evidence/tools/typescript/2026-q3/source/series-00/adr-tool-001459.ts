/** Synthetic decision lifecycle contract for p01-src-001459. */
type DecisionStatus = "proposed" | "reviewed" | "approved" | "released" | "closed";

export interface DecisionRecord { recordId: string; adrId: string; product: string; status: DecisionStatus; service: string; }

export const decision: DecisionRecord = {
  recordId: "p01-src-001459", adrId: "ADR-067", product: "Product Alpha R7", status: "closed", service: "svc-release-coordinator",
};

export function meetsGate(observedP95Ms: number): boolean { return observedP95Ms <= 200 && decision.status !== "proposed"; }
