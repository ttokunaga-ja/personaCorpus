export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence007734: BetaEvidence = {
  sourceId: "p01-src-007734",
  reviewId: "ADR-B4-173",
  layer: "edge adapter",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence007734.targetMs;
}
