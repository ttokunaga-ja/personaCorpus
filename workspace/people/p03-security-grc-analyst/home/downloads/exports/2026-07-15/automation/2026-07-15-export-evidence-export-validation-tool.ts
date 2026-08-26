/** Synthetic defensive validator fixture: p03-full-003622. */
export const recordId = 'p03-full-003622';
export const topic = 'export-evidence-export';
export const spineReferences = [{"id": "org.cobalt-harbor", "value": "Cobalt Harbor Systems, a fictional managed logistics and harbor-operations platform."}, {"id": "measure.evidence-completeness", "value": "Evidence set 184 completeness is 24 of 24 required synthetic items verified."}, {"id": "date.2026-07-15", "value": "Control-owner remediation-plan and retention-disposition validation date."}];
export const assignedCanonicalDates = ["2026-07-15"];
export const exportWindow = 23;
export function completionRate(reviewed: number, completed: number): string {
  return `${(completed / reviewed * 100).toFixed(1)}%`;
}
