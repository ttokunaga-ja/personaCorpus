export type BetaEvidence = Readonly<{ sourceId: string; reviewId: string; layer: string; targetMs: number }> ;

export const evidence008371: BetaEvidence = {
  sourceId: "p01-src-008371",
  reviewId: "ADR-B4-014",
  layer: "command dispatcher",
  targetMs: 225,
};

export function fitsTarget(elapsedMs: number): boolean {
  return elapsedMs <= evidence008371.targetMs;
}
