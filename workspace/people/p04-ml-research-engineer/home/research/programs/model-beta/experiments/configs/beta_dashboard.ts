export type BetaSlice = {
  dimension: "split" | "label" | "confidence_band";
  value: string;
  records: number;
  macroF1: number;
};

export const RUN_ID = "R184";
export const MODEL = "Model Beta B-07";
export const DATASET = "HF-Eval-S1/S2 v3.2";
export const BETA_MACRO_F1 = 0.846;
export const ALPHA_REFERENCE_F1 = 0.873;

export const betaSlices: readonly BetaSlice[] = [
  { dimension: "split", value: "S1", records: 1600, macroF1: 0.852 },
  { dimension: "split", value: "S2", records: 800, macroF1: 0.834 },
  { dimension: "label", value: "refund_request", records: 320, macroF1: 0.816 },
];

export function needsErrorReview(slices: readonly BetaSlice[] = betaSlices): BetaSlice[] {
  return slices.filter((slice) => slice.records >= 100 && slice.macroF1 < BETA_MACRO_F1);
}

export function referenceDelta(): number {
  return Number((BETA_MACRO_F1 - ALPHA_REFERENCE_F1).toFixed(3));
}
