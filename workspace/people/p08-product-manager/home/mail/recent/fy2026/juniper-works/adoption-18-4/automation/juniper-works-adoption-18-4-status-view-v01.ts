/** Reproducible synthetic metric utility for p08-full-005801. */
const artifactId = "p08-full-005801";
const runId = "M-27";
const sliceF1: readonly number[] = [0.872, 0.868, 0.866, 0.881, 0.877, 0.875, 0.871, 0.869];

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
