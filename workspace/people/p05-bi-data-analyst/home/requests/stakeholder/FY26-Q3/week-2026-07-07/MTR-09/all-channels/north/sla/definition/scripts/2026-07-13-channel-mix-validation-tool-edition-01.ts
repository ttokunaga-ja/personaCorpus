/** Synthetic refund patterns helper for Mosaic Retail Analytics. */
export const artifactId = "p05-full-008887";
export const homeRelativePath = "requests/stakeholder/FY26-Q3/week-2026-07-07/MTR-09/all-channels/north/sla/definition/scripts/2026-07-13-channel-mix-validation-tool-edition-01.ts";
export const metric = "MTR-09";
export const gmvJpyM = 84.2;
export const planJpyM = 80.0;

export function metricSummary(): { artifactId: string; metric: string; varianceJpyM: number; focus: string } {
  return { artifactId, metric, varianceJpyM: Number((gmvJpyM - planJpyM).toFixed(1)), focus: "refund patterns" };
}
