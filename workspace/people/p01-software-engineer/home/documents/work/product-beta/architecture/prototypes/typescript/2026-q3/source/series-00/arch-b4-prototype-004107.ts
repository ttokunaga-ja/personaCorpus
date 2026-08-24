export interface BetaArchitectureReview {
  readonly id: string;
  readonly component: string;
  readonly targetP95Ms: number;
}

export const betaReview004107: BetaArchitectureReview = {
  id: "ADR-B4-128",
  component: "event distributor",
  targetP95Ms: 225,
};

export const recordsWithinTarget = (elapsedMs: number): boolean =>
  elapsedMs <= betaReview004107.targetP95Ms;
