export type MigrationReadiness = { recordId: "p01-src-009951"; waveId: "MIG-26Q3-W04"; service: "workflow-engine"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_9951: MigrationReadiness = { recordId: "p01-src-009951", waveId: "MIG-26Q3-W04", service: "workflow-engine", cursorAgeSeconds: 91, restoreSeconds: 2151 };
