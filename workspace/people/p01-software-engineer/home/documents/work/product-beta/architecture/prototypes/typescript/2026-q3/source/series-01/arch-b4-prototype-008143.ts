export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence008143: BetaEvidence = {
  sourceId: "p01-src-008143",
  reviewId: "ADR-B4-184",
  layer: "command dispatcher",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence008143.targetMs;
}
