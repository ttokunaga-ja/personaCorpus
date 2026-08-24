export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence011021: BetaEvidence = {
  sourceId: "p01-src-011021",
  reviewId: "ADR-B4-077",
  layer: "review evidence store",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence011021.targetMs;
}
