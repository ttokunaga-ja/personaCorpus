export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence010569: BetaEvidence = {
  sourceId: "p01-src-010569",
  reviewId: "ADR-B4-023",
  layer: "event relay",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence010569.targetMs;
}
