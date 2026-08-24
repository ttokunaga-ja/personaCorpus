export interface BetaArchitectureReview {
  readonly id: string;
  readonly component: string;
  readonly targetP95Ms: number;
}

export const betaReview004681: BetaArchitectureReview = {
  id: "ADR-B4-105",
  component: "command coordinator",
  targetP95Ms: 225,
};

export const recordsWithinTarget = (elapsedMs: number): boolean =>
  elapsedMs <= betaReview004681.targetP95Ms;
