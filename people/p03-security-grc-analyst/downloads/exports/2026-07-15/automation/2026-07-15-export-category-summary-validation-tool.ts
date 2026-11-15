/** Synthetic defensive validator fixture: p03-full-003660. */
export const recordId = 'p03-full-003660';
export const topic = 'export-category-summary';
export const spineReferences = [{"id": "org.cobalt-harbor", "value": "Cobalt Harbor Systems, a fictional managed logistics and harbor-operations platform."}, {"id": "system.sentinel-export", "value": "Sentinel Export, the fictional CHS minimized SIEM export service."}, {"id": "date.2026-07-15", "value": "Control-owner remediation-plan and retention-disposition validation date."}];
export const assignedCanonicalDates = ["2026-07-15"];
export const exportWindow = 13;
export function completionRate(reviewed: number, completed: number): string {
  return `${(completed / reviewed * 100).toFixed(1)}%`;
}
