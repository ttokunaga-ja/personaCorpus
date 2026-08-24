export type MigrationReadiness = { recordId: "p01-src-001976"; waveId: "MIG-26Q3-W03"; service: "partner-bridge"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_1976: MigrationReadiness = { recordId: "p01-src-001976", waveId: "MIG-26Q3-W03", service: "partner-bridge", cursorAgeSeconds: 256, restoreSeconds: 1976 };
