export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence009020: BetaEvidence = {
  sourceId: "p01-src-009020",
  reviewId: "ADR-B4-066",
  layer: "decision journal",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence009020.targetMs;
}
