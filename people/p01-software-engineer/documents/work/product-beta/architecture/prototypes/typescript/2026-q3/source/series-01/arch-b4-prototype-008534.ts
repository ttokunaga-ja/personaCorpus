export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence008534: BetaEvidence = {
  sourceId: "p01-src-008534",
  reviewId: "ADR-B4-177",
  layer: "decision journal",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence008534.targetMs;
}
