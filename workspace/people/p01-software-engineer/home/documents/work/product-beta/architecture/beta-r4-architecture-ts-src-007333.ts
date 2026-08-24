export interface BetaArchitectureReview {
  readonly id: string;
  readonly component: string;
  readonly targetP95Ms: number;
}

export const betaReview007333: BetaArchitectureReview = {
  id: "ADR-B4-170",
  component: "command coordinator",
  targetP95Ms: 225,
};

export const recordsWithinTarget = (elapsedMs: number): boolean =>
  elapsedMs <= betaReview007333.targetP95Ms;
