export interface BetaArchitectureReview {
  readonly id: string;
  readonly component: string;
  readonly targetP95Ms: number;
}

export const betaReview002808: BetaArchitectureReview = {
  id: "ADR-B4-023",
  component: "ingress adapter",
  targetP95Ms: 225,
};

export const recordsWithinTarget = (elapsedMs: number): boolean =>
  elapsedMs <= betaReview002808.targetP95Ms;
