export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence010881: BetaEvidence = {
  sourceId: "p01-src-010881",
  reviewId: "ADR-B4-136",
  layer: "event relay",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence010881.targetMs;
}
