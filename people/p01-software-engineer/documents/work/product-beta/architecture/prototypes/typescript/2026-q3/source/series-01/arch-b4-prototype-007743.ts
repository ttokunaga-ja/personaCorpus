export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence007743: BetaEvidence = {
  sourceId: "p01-src-007743",
  reviewId: "ADR-B4-182",
  layer: "event relay",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence007743.targetMs;
}
