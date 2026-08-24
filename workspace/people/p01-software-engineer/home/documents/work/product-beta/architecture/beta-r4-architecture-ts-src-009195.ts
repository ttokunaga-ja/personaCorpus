export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence009195: BetaEvidence = {
  sourceId: "p01-src-009195",
  reviewId: "ADR-B4-042",
  layer: "event relay",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence009195.targetMs;
}
