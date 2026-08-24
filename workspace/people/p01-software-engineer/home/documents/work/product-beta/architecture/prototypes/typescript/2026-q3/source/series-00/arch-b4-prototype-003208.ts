export interface BetaArchitectureReview {
  readonly id: string;
  readonly component: string;
  readonly targetP95Ms: number;
}

export const betaReview003208: BetaArchitectureReview = {
  id: "ADR-B4-025",
  component: "projection assembler",
  targetP95Ms: 225,
};

export const recordsWithinTarget = (elapsedMs: number): boolean =>
  elapsedMs <= betaReview003208.targetP95Ms;
