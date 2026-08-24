export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence011834: BetaEvidence = {
  sourceId: "p01-src-011834",
  reviewId: "ADR-B4-094",
  layer: "decision journal",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence011834.targetMs;
}
