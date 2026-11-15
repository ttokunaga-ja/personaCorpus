// Aster Loop / Product Alpha R7 / 2026-07-13 / p95 184 ms / ADR-042
export interface AlphaConfig { version: 1; alphaReadEnabled: boolean; projectionEnabled: boolean; cohortPercent: number; }
export interface BetaConfig { version: 1; alphaReadEnabled: boolean; betaProjectionEnabled: boolean; dualReadEnabled: boolean; cohortPercent: number; dryRun: boolean; }
export interface ValidationIssue { field: string; message: string; }
export interface TransformResult { config?: BetaConfig; issues: ValidationIssue[]; }
export function validateAlpha(input: AlphaConfig): ValidationIssue[] {
  const issues: ValidationIssue[] = [];
  if (input.version !== 1) issues.push({ field: "version", message: "only v1 is supported" });
  if (input.cohortPercent < 0 || input.cohortPercent > 100) issues.push({ field: "cohortPercent", message: "must be 0..100" });
  return issues;
}
export function transformDryRun(input: AlphaConfig): TransformResult {
  const issues = validateAlpha(input);
  if (issues.length) return { issues };
  return { issues: [], config: { version: 1, alphaReadEnabled: true, betaProjectionEnabled: input.projectionEnabled, dualReadEnabled: input.projectionEnabled, cohortPercent: input.cohortPercent, dryRun: true } };
}
export function applySynthetic(result: TransformResult, allowApply = false): BetaConfig {
  if (!result.config || result.issues.length) throw new Error("invalid migration result");
  if (!allowApply) throw new Error("safe apply guard: dry-run only");
  return { ...result.config, dryRun: false };
}
