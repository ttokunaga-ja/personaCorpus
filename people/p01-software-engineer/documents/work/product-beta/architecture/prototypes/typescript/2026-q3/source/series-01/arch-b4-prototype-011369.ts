export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence011369: BetaEvidence = {
  sourceId: "p01-src-011369",
  reviewId: "ADR-B4-027",
  layer: "review evidence store",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence011369.targetMs;
}
