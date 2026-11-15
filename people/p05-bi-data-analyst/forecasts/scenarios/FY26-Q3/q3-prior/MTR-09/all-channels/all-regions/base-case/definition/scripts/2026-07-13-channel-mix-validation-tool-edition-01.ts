/** Synthetic GMV performance helper for Mosaic Retail Analytics. */
export const artifactId = "p05-full-004428";
export const homeRelativePath = "forecasts/scenarios/FY26-Q3/q3-prior/MTR-09/all-channels/all-regions/base-case/definition/scripts/2026-07-13-channel-mix-validation-tool-edition-01.ts";
export const metric = "MTR-09";
export const gmvJpyM = 84.2;
export const planJpyM = 80.0;

export function metricSummary(): { artifactId: string; metric: string; varianceJpyM: number; focus: string } {
  return { artifactId, metric, varianceJpyM: Number((gmvJpyM - planJpyM).toFixed(1)), focus: "GMV performance" };
}
