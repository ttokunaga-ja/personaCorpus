export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence008934: BetaEvidence = {
  sourceId: "p01-src-008934",
  reviewId: "ADR-B4-179",
  layer: "edge adapter",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence008934.targetMs;
}
