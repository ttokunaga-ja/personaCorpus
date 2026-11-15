export interface BetaArchitectureReview {
  readonly id: string;
  readonly component: string;
  readonly targetP95Ms: number;
}

export const betaReview002995: BetaArchitectureReview = {
  id: "ADR-B4-011",
  component: "command coordinator",
  targetP95Ms: 225,
};

export const recordsWithinTarget = (elapsedMs: number): boolean =>
  elapsedMs <= betaReview002995.targetP95Ms;
