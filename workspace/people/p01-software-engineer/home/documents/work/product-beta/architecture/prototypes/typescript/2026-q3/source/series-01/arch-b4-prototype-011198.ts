export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence011198: BetaEvidence = {
  sourceId: "p01-src-011198",
  reviewId: "ADR-B4-055",
  layer: "decision journal",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence011198.targetMs;
}
