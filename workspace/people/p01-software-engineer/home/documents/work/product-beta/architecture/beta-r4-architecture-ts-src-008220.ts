export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence008220: BetaEvidence = {
  sourceId: "p01-src-008220",
  reviewId: "ADR-B4-062",
  layer: "edge adapter",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence008220.targetMs;
}
