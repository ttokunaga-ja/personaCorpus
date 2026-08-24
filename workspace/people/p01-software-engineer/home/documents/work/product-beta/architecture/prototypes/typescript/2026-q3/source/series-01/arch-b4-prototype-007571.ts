export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence007571: BetaEvidence = {
  sourceId: "p01-src-007571",
  reviewId: "ADR-B4-010",
  layer: "review evidence store",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence007571.targetMs;
}
