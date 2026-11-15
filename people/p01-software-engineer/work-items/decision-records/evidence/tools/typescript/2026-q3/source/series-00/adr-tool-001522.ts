/** Synthetic decision lifecycle contract for p01-src-001522. */
type DecisionStatus = "proposed" | "reviewed" | "approved" | "released" | "closed";

export interface DecisionRecord { recordId: string; adrId: string; product: string; status: DecisionStatus; service: string; }

export const decision: DecisionRecord = {
  recordId: "p01-src-001522", adrId: "ADR-130", product: "Product Alpha R7", status: "approved", service: "svc-observability",
};

export function meetsGate(observedP95Ms: number): boolean { return observedP95Ms <= 200 && decision.status !== "proposed"; }
