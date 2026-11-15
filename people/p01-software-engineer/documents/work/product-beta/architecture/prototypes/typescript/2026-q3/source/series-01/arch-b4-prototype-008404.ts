export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence008404: BetaEvidence = {
  sourceId: "p01-src-008404",
  reviewId: "ADR-B4-047",
  layer: "projection reader",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence008404.targetMs;
}
