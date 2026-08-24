export interface BetaArchitectureReview {
  readonly id: string;
  readonly component: string;
  readonly targetP95Ms: number;
}

export const betaReview007181: BetaArchitectureReview = {
  id: "ADR-B4-018",
  component: "review evidence sink",
  targetP95Ms: 225,
};

export const recordsWithinTarget = (elapsedMs: number): boolean =>
  elapsedMs <= betaReview007181.targetP95Ms;
