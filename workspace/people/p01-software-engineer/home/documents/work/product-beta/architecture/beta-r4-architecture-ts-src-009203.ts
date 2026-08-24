export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence009203: BetaEvidence = {
  sourceId: "p01-src-009203",
  reviewId: "ADR-B4-050",
  layer: "review evidence store",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence009203.targetMs;
}
