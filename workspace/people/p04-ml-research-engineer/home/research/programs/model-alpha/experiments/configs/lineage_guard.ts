export type LineageRecord = {
  runId: string;
  evaluationDate: string;
  model: string;
  dataset: string;
  records: number;
  macroF1: number;
};

const expected: LineageRecord = {
  runId: "R184", evaluationDate: "2026-07-13", model: "Model Alpha M-14",
  dataset: "HF-Eval-S1/S2 v3.2", records: 2400, macroF1: 0.873,
};

export function guard(record: LineageRecord): string[] {
  const failures: string[] = [];
  for (const key of Object.keys(expected) as (keyof LineageRecord)[]) {
    if (record[key] !== expected[key]) failures.push(`lineage mismatch: ${key}`);
  }
  return failures;
}

export function referenceMetric(): string {
  return `run R184 reference F1 ${expected.macroF1.toFixed(3)}`;
}
