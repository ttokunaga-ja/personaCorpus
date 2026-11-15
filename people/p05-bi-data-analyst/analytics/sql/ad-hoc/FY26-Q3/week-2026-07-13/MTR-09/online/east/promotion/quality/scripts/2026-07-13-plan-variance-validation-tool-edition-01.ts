/** Synthetic refresh controls helper for Mosaic Retail Analytics. */
export const artifactId = "p05-full-008632";
export const homeRelativePath = "analytics/sql/ad-hoc/FY26-Q3/week-2026-07-13/MTR-09/online/east/promotion/quality/scripts/2026-07-13-plan-variance-validation-tool-edition-01.ts";
export const metric = "MTR-09";
export const gmvJpyM = 84.2;
export const planJpyM = 80.0;

export function metricSummary(): { artifactId: string; metric: string; varianceJpyM: number; focus: string } {
  return { artifactId, metric, varianceJpyM: Number((gmvJpyM - planJpyM).toFixed(1)), focus: "refresh controls" };
}
