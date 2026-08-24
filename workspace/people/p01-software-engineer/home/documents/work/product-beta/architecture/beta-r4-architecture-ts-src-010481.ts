export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence010481: BetaEvidence = {
  sourceId: "p01-src-010481",
  reviewId: "ADR-B4-134",
  layer: "review evidence store",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence010481.targetMs;
}
