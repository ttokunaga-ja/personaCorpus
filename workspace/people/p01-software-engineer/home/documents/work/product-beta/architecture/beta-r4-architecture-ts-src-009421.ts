export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence009421: BetaEvidence = {
  sourceId: "p01-src-009421",
  reviewId: "ADR-B4-069",
  layer: "command dispatcher",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence009421.targetMs;
}
