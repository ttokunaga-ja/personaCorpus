export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence008804: BetaEvidence = {
  sourceId: "p01-src-008804",
  reviewId: "ADR-B4-049",
  layer: "decision journal",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence008804.targetMs;
}
