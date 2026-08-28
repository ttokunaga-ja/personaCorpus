/** Reproducible synthetic metric utility for p04-full-003604. */
const artifactId = "p04-full-003604";
const runId = "R184";
const sliceF1: readonly number[] = [0.879, 0.858, 0.873, 0.870, 0.867, 0.864, 0.879, 0.858];

export type MetricSummary = {
  artifactId: string;
  runId: string;
  meanF1: number;
  minF1: number;
  maxF1: number;
};

export function summarize(values: readonly number[]): MetricSummary {
  if (values.length === 0) throw new Error("values must not be empty");
  const mean = values.reduce((total, value) => total + value, 0) / values.length;
  return {
    artifactId,
    runId,
    meanF1: Number(mean.toFixed(3)),
    minF1: Math.min(...values),
    maxF1: Math.max(...values),
  };
}

console.log(JSON.stringify(summarize(sliceF1)));
