export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence007820: BetaEvidence = {
  sourceId: "p01-src-007820",
  reviewId: "ADR-B4-060",
  layer: "decision journal",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence007820.targetMs;
}
