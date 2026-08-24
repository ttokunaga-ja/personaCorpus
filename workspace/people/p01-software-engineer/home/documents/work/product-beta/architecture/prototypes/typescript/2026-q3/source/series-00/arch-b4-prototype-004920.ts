export interface BetaArchitectureReview {
  readonly id: string;
  readonly component: string;
  readonly targetP95Ms: number;
}

export const betaReview004920: BetaArchitectureReview = {
  id: "ADR-B4-145",
  component: "ingress adapter",
  targetP95Ms: 225,
};

export const recordsWithinTarget = (elapsedMs: number): boolean =>
  elapsedMs <= betaReview004920.targetP95Ms;
