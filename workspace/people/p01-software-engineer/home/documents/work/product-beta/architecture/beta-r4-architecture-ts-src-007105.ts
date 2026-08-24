export interface BetaArchitectureReview {
  readonly id: string;
  readonly component: string;
  readonly targetP95Ms: number;
}

export const betaReview007105: BetaArchitectureReview = {
  id: "ADR-B4-141",
  component: "command coordinator",
  targetP95Ms: 225,
};

export const recordsWithinTarget = (elapsedMs: number): boolean =>
  elapsedMs <= betaReview007105.targetP95Ms;
