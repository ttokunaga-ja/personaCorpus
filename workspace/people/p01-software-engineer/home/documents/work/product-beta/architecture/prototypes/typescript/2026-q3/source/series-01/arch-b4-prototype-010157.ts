export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence010157: BetaEvidence = {
  sourceId: "p01-src-010157",
  reviewId: "ADR-B4-009",
  layer: "review evidence store",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence010157.targetMs;
}
