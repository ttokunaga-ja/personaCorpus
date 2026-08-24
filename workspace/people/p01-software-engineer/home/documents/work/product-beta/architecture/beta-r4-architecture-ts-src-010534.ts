export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence010534: BetaEvidence = {
  sourceId: "p01-src-010534",
  reviewId: "ADR-B4-187",
  layer: "projection reader",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence010534.targetMs;
}
