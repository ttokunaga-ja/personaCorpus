export interface BetaArchitectureReview {
  readonly id: string;
  readonly component: string;
  readonly targetP95Ms: number;
}

export const betaReview005373: BetaArchitectureReview = {
  id: "ADR-B4-001",
  component: "event distributor",
  targetP95Ms: 225,
};

export const recordsWithinTarget = (elapsedMs: number): boolean =>
  elapsedMs <= betaReview005373.targetP95Ms;
