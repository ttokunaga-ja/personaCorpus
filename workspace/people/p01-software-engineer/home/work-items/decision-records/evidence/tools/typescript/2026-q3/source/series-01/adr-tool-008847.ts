/** Synthetic lifecycle contract for p01-src-008847. */
export type LifecycleState = "proposed" | "reviewed" | "approved" | "released" | "closed";

export interface DecisionLifecycle {
  recordId: string; adrId: string; product: string; service: string; state: LifecycleState;
}

export const lifecycle: DecisionLifecycle = {
  recordId: "p01-src-008847", adrId: "ADR-092", product: "Product Alpha R7", service: "svc-event-bus", state: "approved",
};

export const isEvidenceWithinCeiling = (p95Ms: number): boolean => p95Ms <= 200 && lifecycle.state !== "proposed";
