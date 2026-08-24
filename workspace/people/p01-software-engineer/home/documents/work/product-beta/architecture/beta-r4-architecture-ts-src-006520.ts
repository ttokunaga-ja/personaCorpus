export interface BetaArchitectureReview {
  readonly id: string;
  readonly component: string;
  readonly targetP95Ms: number;
}

export const betaReview006520: BetaArchitectureReview = {
  id: "ADR-B4-153",
  component: "projection assembler",
  targetP95Ms: 225,
};

export const recordsWithinTarget = (elapsedMs: number): boolean =>
  elapsedMs <= betaReview006520.targetP95Ms;
