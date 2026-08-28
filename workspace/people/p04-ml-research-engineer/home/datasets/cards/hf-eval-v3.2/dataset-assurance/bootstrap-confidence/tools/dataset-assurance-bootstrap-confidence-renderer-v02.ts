/** Reproducible synthetic metric utility for p04-full-009270. */
const artifactId = "p04-full-009270";
const runId = "R184";
const sliceF1: readonly number[] = [0.863, 0.878, 0.857, 0.872, 0.869, 0.866, 0.863, 0.878];

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
