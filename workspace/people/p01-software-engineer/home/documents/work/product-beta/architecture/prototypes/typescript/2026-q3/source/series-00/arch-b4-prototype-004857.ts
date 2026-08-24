export interface BetaArchitectureReview {
  readonly id: string;
  readonly component: string;
  readonly targetP95Ms: number;
}

export const betaReview004857: BetaArchitectureReview = {
  id: "ADR-B4-082",
  component: "event distributor",
  targetP95Ms: 225,
};

export const recordsWithinTarget = (elapsedMs: number): boolean =>
  elapsedMs <= betaReview004857.targetP95Ms;
