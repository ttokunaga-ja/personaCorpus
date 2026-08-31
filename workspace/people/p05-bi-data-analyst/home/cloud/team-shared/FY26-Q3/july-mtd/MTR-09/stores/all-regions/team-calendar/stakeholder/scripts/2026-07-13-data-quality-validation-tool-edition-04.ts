/** Synthetic refund patterns helper for Mosaic Retail Analytics. */
export const artifactId = "p05-full-010201";
export const homeRelativePath = "cloud/team-shared/FY26-Q3/july-mtd/MTR-09/stores/all-regions/team-calendar/stakeholder/scripts/2026-07-13-data-quality-validation-tool-edition-04.ts";
export const metric = "MTR-09";
export const gmvJpyM = 84.2;
export const planJpyM = 80.0;

export function metricSummary(): { artifactId: string; metric: string; varianceJpyM: number; focus: string } {
  return { artifactId, metric, varianceJpyM: Number((gmvJpyM - planJpyM).toFixed(1)), focus: "refund patterns" };
}
