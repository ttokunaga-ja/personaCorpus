export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence009821: BetaEvidence = {
  sourceId: "p01-src-009821",
  reviewId: "ADR-B4-071",
  layer: "review evidence store",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence009821.targetMs;
}
