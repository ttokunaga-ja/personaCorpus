export interface BetaArchitectureReview {
  readonly id: string;
  readonly component: string;
  readonly targetP95Ms: number;
}

export const betaReview003242: BetaArchitectureReview = {
  id: "ADR-B4-059",
  component: "decision ledger",
  targetP95Ms: 225,
};

export const recordsWithinTarget = (elapsedMs: number): boolean =>
  elapsedMs <= betaReview003242.targetP95Ms;
