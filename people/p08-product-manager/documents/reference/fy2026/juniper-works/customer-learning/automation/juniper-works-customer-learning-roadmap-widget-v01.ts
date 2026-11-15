/** Reproducible synthetic metric utility for p08-full-006389. */
const artifactId = "p08-full-006389";
const runId = "M-27";
const sliceF1: readonly number[] = [0.869, 0.884, 0.863, 0.878, 0.876, 0.872, 0.870, 0.885];

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
