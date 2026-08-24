export interface BetaArchitectureReview {
  readonly id: string;
  readonly component: string;
  readonly targetP95Ms: number;
}

export const betaReview002831: BetaArchitectureReview = {
  id: "ADR-B4-046",
  component: "review evidence sink",
  targetP95Ms: 225,
};

export const recordsWithinTarget = (elapsedMs: number): boolean =>
  elapsedMs <= betaReview002831.targetP95Ms;
