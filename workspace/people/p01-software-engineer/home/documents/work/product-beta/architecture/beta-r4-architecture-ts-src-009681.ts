export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence009681: BetaEvidence = {
  sourceId: "p01-src-009681",
  reviewId: "ADR-B4-130",
  layer: "event relay",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence009681.targetMs;
}
