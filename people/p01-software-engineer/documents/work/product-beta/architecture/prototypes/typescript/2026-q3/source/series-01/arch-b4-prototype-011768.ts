export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence011768: BetaEvidence = {
  sourceId: "p01-src-011768",
  reviewId: "ADR-B4-028",
  layer: "decision journal",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence011768.targetMs;
}
