/** Synthetic channel mix helper for Mosaic Retail Analytics. */
export const artifactId = "p05-full-010605";
export const homeRelativePath = "dashboards/sales/FY26-Q3/week-2026-07-13/MTR-09/stores/all-regions/sales/monthly/scripts/2026-07-13-region-mix-validation-tool-edition-02.ts";
export const metric = "MTR-09";
export const gmvJpyM = 84.2;
export const planJpyM = 80.0;

export function metricSummary(): { artifactId: string; metric: string; varianceJpyM: number; focus: string } {
  return { artifactId, metric, varianceJpyM: Number((gmvJpyM - planJpyM).toFixed(1)), focus: "channel mix" };
}
