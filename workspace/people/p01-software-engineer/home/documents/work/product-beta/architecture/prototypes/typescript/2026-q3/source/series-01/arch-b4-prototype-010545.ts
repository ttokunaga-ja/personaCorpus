export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence010545: BetaEvidence = {
  sourceId: "p01-src-010545",
  reviewId: "ADR-B4-198",
  layer: "event relay",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence010545.targetMs;
}
