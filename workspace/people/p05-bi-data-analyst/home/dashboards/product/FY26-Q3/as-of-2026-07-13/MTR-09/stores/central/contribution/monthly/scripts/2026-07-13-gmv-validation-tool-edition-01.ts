/** Synthetic refresh controls helper for Mosaic Retail Analytics. */
export const artifactId = "p05-full-003466";
export const homeRelativePath = "dashboards/product/FY26-Q3/as-of-2026-07-13/MTR-09/stores/central/contribution/monthly/scripts/2026-07-13-gmv-validation-tool-edition-01.ts";
export const metric = "MTR-09";
export const gmvJpyM = 84.2;
export const planJpyM = 80.0;

export function metricSummary(): { artifactId: string; metric: string; varianceJpyM: number; focus: string } {
  return { artifactId, metric, varianceJpyM: Number((gmvJpyM - planJpyM).toFixed(1)), focus: "refresh controls" };
}
