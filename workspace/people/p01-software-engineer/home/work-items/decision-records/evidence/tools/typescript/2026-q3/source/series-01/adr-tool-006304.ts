/** Synthetic lifecycle contract for p01-src-006304. */
export type LifecycleState = "proposed" | "reviewed" | "approved" | "released" | "closed";

export interface DecisionLifecycle {
  recordId: string; adrId: string; product: string; service: string; state: LifecycleState;
}

export const lifecycle: DecisionLifecycle = {
  recordId: "p01-src-006304", adrId: "ADR-B4-136", product: "Product Beta R4", service: "svc-observability", state: "closed",
};

export const isEvidenceWithinCeiling = (p95Ms: number): boolean => p95Ms <= 200 && lifecycle.state !== "proposed";
