/** Synthetic channel mix helper for Mosaic Retail Analytics. */
export const artifactId = "p05-full-007209";
export const homeRelativePath = "mail/recent/FY26-Q3/q3-plan/MTR-09/online/west/completion/quality/scripts/2026-07-13-gmv-validation-tool-edition-01.ts";
export const metric = "MTR-09";
export const gmvJpyM = 84.2;
export const planJpyM = 80.0;

export function metricSummary(): { artifactId: string; metric: string; varianceJpyM: number; focus: string } {
  return { artifactId, metric, varianceJpyM: Number((gmvJpyM - planJpyM).toFixed(1)), focus: "channel mix" };
}
