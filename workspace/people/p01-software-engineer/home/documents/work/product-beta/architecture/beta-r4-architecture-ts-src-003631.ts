export interface BetaArchitectureReview {
  readonly id: string;
  readonly component: string;
  readonly targetP95Ms: number;
}

export const betaReview003631: BetaArchitectureReview = {
  id: "ADR-B4-050",
  component: "command coordinator",
  targetP95Ms: 225,
};

export const recordsWithinTarget = (elapsedMs: number): boolean =>
  elapsedMs <= betaReview003631.targetP95Ms;
