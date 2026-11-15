export interface BetaArchitectureReview {
  readonly id: string;
  readonly component: string;
  readonly targetP95Ms: number;
}

export const betaReview003642: BetaArchitectureReview = {
  id: "ADR-B4-061",
  component: "ingress adapter",
  targetP95Ms: 225,
};

export const recordsWithinTarget = (elapsedMs: number): boolean =>
  elapsedMs <= betaReview003642.targetP95Ms;
