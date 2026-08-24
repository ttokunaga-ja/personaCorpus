export interface BetaArchitectureReview {
  readonly id: string;
  readonly component: string;
  readonly targetP95Ms: number;
}

export const betaReview004042: BetaArchitectureReview = {
  id: "ADR-B4-063",
  component: "projection assembler",
  targetP95Ms: 225,
};

export const recordsWithinTarget = (elapsedMs: number): boolean =>
  elapsedMs <= betaReview004042.targetP95Ms;
