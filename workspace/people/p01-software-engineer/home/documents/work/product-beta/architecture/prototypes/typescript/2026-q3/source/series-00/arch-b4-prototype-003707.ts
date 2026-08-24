export interface BetaArchitectureReview {
  readonly id: string;
  readonly component: string;
  readonly targetP95Ms: number;
}

export const betaReview003707: BetaArchitectureReview = {
  id: "ADR-B4-126",
  component: "review evidence sink",
  targetP95Ms: 225,
};

export const recordsWithinTarget = (elapsedMs: number): boolean =>
  elapsedMs <= betaReview003707.targetP95Ms;
