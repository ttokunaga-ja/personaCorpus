/** Synthetic lifecycle contract for p01-src-008706. */
export type LifecycleState = "proposed" | "reviewed" | "approved" | "released" | "closed";

export interface DecisionLifecycle {
  recordId: string; adrId: string; product: string; service: string; state: LifecycleState;
}

export const lifecycle: DecisionLifecycle = {
  recordId: "p01-src-008706", adrId: "ADR-B4-150", product: "Product Beta R4", service: "svc-edge-api", state: "reviewed",
};

export const isEvidenceWithinCeiling = (p95Ms: number): boolean => p95Ms <= 200 && lifecycle.state !== "proposed";
