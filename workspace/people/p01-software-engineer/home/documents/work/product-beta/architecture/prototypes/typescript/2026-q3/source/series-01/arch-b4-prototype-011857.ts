export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence011857: BetaEvidence = {
  sourceId: "p01-src-011857",
  reviewId: "ADR-B4-117",
  layer: "command dispatcher",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence011857.targetMs;
}
