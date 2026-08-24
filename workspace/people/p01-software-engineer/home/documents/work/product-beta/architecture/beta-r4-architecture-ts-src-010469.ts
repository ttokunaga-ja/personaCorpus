export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence010469: BetaEvidence = {
  sourceId: "p01-src-010469",
  reviewId: "ADR-B4-122",
  layer: "review evidence store",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence010469.targetMs;
}
