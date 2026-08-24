export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence009996: BetaEvidence = {
  sourceId: "p01-src-009996",
  reviewId: "ADR-B4-047",
  layer: "edge adapter",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence009996.targetMs;
}
