export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence008944: BetaEvidence = {
  sourceId: "p01-src-008944",
  reviewId: "ADR-B4-189",
  layer: "projection reader",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence008944.targetMs;
}
