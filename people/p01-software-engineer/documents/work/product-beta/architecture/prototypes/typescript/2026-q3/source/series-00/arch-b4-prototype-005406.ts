export interface BetaArchitectureReview {
  readonly id: string;
  readonly component: string;
  readonly targetP95Ms: number;
}

export const betaReview005406: BetaArchitectureReview = {
  id: "ADR-B4-034",
  component: "ingress adapter",
  targetP95Ms: 225,
};

export const recordsWithinTarget = (elapsedMs: number): boolean =>
  elapsedMs <= betaReview005406.targetP95Ms;
