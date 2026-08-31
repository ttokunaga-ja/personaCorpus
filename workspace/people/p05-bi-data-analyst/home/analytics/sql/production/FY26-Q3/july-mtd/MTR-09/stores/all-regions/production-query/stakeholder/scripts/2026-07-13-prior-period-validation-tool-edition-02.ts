/** Synthetic plan variance helper for Mosaic Retail Analytics. */
export const artifactId = "p05-full-000482";
export const homeRelativePath = "analytics/sql/production/FY26-Q3/july-mtd/MTR-09/stores/all-regions/production-query/stakeholder/scripts/2026-07-13-prior-period-validation-tool-edition-02.ts";
export const metric = "MTR-09";
export const gmvJpyM = 84.2;
export const planJpyM = 80.0;

export function metricSummary(): { artifactId: string; metric: string; varianceJpyM: number; focus: string } {
  return { artifactId, metric, varianceJpyM: Number((gmvJpyM - planJpyM).toFixed(1)), focus: "plan variance" };
}
