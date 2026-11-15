export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence007343: BetaEvidence = {
  sourceId: "p01-src-007343",
  reviewId: "ADR-B4-180",
  layer: "review evidence store",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence007343.targetMs;
}
