export interface BetaArchitectureReview {
  readonly id: string;
  readonly component: string;
  readonly targetP95Ms: number;
}

export const betaReview005720: BetaArchitectureReview = {
  id: "ADR-B4-149",
  component: "decision ledger",
  targetP95Ms: 225,
};

export const recordsWithinTarget = (elapsedMs: number): boolean =>
  elapsedMs <= betaReview005720.targetP95Ms;
