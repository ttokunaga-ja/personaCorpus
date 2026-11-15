export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence011998: BetaEvidence = {
  sourceId: "p01-src-011998",
  reviewId: "ADR-B4-059",
  layer: "projection reader",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence011998.targetMs;
}
