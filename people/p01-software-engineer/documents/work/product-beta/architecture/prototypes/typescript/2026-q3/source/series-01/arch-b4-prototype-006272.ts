export interface BetaArchitectureReview {
  readonly id: string;
  readonly component: string;
  readonly targetP95Ms: number;
}

export const betaReview006272: BetaArchitectureReview = {
  id: "ADR-B4-104",
  component: "decision ledger",
  targetP95Ms: 225,
};

export const recordsWithinTarget = (elapsedMs: number): boolean =>
  elapsedMs <= betaReview006272.targetP95Ms;
