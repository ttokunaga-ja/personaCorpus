export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence007420: BetaEvidence = {
  sourceId: "p01-src-007420",
  reviewId: "ADR-B4-058",
  layer: "projection reader",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence007420.targetMs;
}
