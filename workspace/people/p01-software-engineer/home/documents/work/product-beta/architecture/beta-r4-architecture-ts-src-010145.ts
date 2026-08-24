export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence010145: BetaEvidence = {
  sourceId: "p01-src-010145",
  reviewId: "ADR-B4-196",
  layer: "review evidence store",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence010145.targetMs;
}
