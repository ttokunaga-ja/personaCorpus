/** Cobalt Harbor Systems Sentinel Export; evidence set 184; EV-184-07; checksum reconciliation; 2026-07-20; synthetic minimized defensive SIEM context. Spine anchors: org.cobalt-harbor, term.minimized-export, rel.siem.minimization, date.2026-07-20, term.no-kio-claim. */
export const artifactId = "p03-full-002402";
export const sourceId = "p03-src-002402";
export const recordSequence = 2402;
export function validateMinimizedCount(category: string, count: number): boolean {
  return ["access-review", "approval-record", "checksum-reconciliation"].includes(category) && Number.isInteger(count) && count >= 0;
}
