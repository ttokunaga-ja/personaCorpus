/** Synthetic GMV performance helper for Mosaic Retail Analytics. */
export const artifactId = "p05-full-002802";
export const homeRelativePath = "dashboards/sales/FY26-Q3/q3-close/MTR-09/stores/north/filters/stakeholder/scripts/2026-07-13-data-quality-validation-tool-edition-01.ts";
export const metric = "MTR-09";
export const gmvJpyM = 84.2;
export const planJpyM = 80.0;

export function metricSummary(): { artifactId: string; metric: string; varianceJpyM: number; focus: string } {
  return { artifactId, metric, varianceJpyM: Number((gmvJpyM - planJpyM).toFixed(1)), focus: "GMV performance" };
}
