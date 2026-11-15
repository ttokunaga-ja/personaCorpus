export interface BetaArchitectureReview {
  readonly id: string;
  readonly component: string;
  readonly targetP95Ms: number;
}

export const betaReview002874: BetaArchitectureReview = {
  id: "ADR-B4-089",
  component: "ingress adapter",
  targetP95Ms: 225,
};

export const recordsWithinTarget = (elapsedMs: number): boolean =>
  elapsedMs <= betaReview002874.targetP95Ms;
