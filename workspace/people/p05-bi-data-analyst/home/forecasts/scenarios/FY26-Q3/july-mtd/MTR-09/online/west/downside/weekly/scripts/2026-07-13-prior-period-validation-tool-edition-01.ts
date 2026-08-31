/** Synthetic plan variance helper for Mosaic Retail Analytics. */
export const artifactId = "p05-full-004634";
export const homeRelativePath = "forecasts/scenarios/FY26-Q3/july-mtd/MTR-09/online/west/downside/weekly/scripts/2026-07-13-prior-period-validation-tool-edition-01.ts";
export const metric = "MTR-09";
export const gmvJpyM = 84.2;
export const planJpyM = 80.0;

export function metricSummary(): { artifactId: string; metric: string; varianceJpyM: number; focus: string } {
  return { artifactId, metric, varianceJpyM: Number((gmvJpyM - planJpyM).toFixed(1)), focus: "plan variance" };
}
