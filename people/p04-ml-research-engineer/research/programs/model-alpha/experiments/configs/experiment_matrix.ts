export type MatrixArm = {
  id: "A0" | "A1" | "A2" | "A3";
  change: string;
  expectedMacroF1: number;
};

export const RUN_ID = "R184";
export const MODEL = "Model Alpha M-14";
export const DATASET = "HF-Eval-S1/S2 v3.2";

export const alphaMatrix: readonly MatrixArm[] = [
  { id: "A0", change: "full configuration", expectedMacroF1: 0.873 },
  { id: "A1", change: "disable class weights", expectedMacroF1: 0.858 },
  { id: "A2", change: "disable temperature calibration", expectedMacroF1: 0.869 },
  { id: "A3", change: "truncate at 96 tokens", expectedMacroF1: 0.864 },
];

export function bestArm(arms: readonly MatrixArm[] = alphaMatrix): MatrixArm {
  return arms.reduce((best, candidate) => candidate.expectedMacroF1 > best.expectedMacroF1 ? candidate : best);
}
