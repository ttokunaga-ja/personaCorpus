export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence008295: BetaEvidence = {
  sourceId: "p01-src-008295",
  reviewId: "ADR-B4-137",
  layer: "event relay",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence008295.targetMs;
}
