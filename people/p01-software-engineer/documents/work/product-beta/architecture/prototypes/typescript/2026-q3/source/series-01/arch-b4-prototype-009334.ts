export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence009334: BetaEvidence = {
  sourceId: "p01-src-009334",
  reviewId: "ADR-B4-181",
  layer: "projection reader",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence009334.targetMs;
}
