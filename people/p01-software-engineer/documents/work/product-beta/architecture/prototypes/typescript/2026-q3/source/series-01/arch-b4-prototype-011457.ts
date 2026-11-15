export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence011457: BetaEvidence = {
  sourceId: "p01-src-011457",
  reviewId: "ADR-B4-115",
  layer: "event relay",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence011457.targetMs;
}
