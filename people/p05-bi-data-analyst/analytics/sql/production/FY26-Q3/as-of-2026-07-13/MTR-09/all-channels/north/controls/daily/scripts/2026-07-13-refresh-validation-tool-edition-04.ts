/** Synthetic refund patterns helper for Mosaic Retail Analytics. */
export const artifactId = "p05-full-009799";
export const homeRelativePath = "analytics/sql/production/FY26-Q3/as-of-2026-07-13/MTR-09/all-channels/north/controls/daily/scripts/2026-07-13-refresh-validation-tool-edition-04.ts";
export const metric = "MTR-09";
export const gmvJpyM = 84.2;
export const planJpyM = 80.0;

export function metricSummary(): { artifactId: string; metric: string; varianceJpyM: number; focus: string } {
  return { artifactId, metric, varianceJpyM: Number((gmvJpyM - planJpyM).toFixed(1)), focus: "refund patterns" };
}
