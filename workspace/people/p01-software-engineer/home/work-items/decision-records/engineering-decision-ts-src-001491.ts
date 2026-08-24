/** Synthetic decision lifecycle contract for p01-src-001491. */
type DecisionStatus = "proposed" | "reviewed" | "approved" | "released" | "closed";

export interface DecisionRecord { recordId: string; adrId: string; product: string; status: DecisionStatus; service: string; }

export const decision: DecisionRecord = {
  recordId: "p01-src-001491", adrId: "ADR-B4-099", product: "Product Beta R4", status: "reviewed", service: "svc-event-bus",
};

export function meetsGate(observedP95Ms: number): boolean { return observedP95Ms <= 200 && decision.status !== "proposed"; }
