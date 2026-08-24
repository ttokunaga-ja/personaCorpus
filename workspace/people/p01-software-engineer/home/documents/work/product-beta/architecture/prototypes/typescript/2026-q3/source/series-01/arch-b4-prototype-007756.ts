export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence007756: BetaEvidence = {
  sourceId: "p01-src-007756",
  reviewId: "ADR-B4-195",
  layer: "projection reader",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence007756.targetMs;
}
