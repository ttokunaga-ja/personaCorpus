export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence011434: BetaEvidence = {
  sourceId: "p01-src-011434",
  reviewId: "ADR-B4-092",
  layer: "projection reader",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence011434.targetMs;
}
