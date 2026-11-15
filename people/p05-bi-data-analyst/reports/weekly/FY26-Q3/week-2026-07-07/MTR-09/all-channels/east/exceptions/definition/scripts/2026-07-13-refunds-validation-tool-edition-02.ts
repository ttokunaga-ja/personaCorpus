/** Synthetic refresh controls helper for Mosaic Retail Analytics. */
export const artifactId = "p05-full-003922";
export const homeRelativePath = "reports/weekly/FY26-Q3/week-2026-07-07/MTR-09/all-channels/east/exceptions/definition/scripts/2026-07-13-refunds-validation-tool-edition-02.ts";
export const metric = "MTR-09";
export const gmvJpyM = 84.2;
export const planJpyM = 80.0;

export function metricSummary(): { artifactId: string; metric: string; varianceJpyM: number; focus: string } {
  return { artifactId, metric, varianceJpyM: Number((gmvJpyM - planJpyM).toFixed(1)), focus: "refresh controls" };
}
