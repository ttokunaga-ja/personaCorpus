export type MigrationReadiness = { recordId: "p01-src-005882"; waveId: "MIG-26Q3-W03"; service: "partner-bridge"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_5882: MigrationReadiness = { recordId: "p01-src-005882", waveId: "MIG-26Q3-W03", service: "partner-bridge", cursorAgeSeconds: 202, restoreSeconds: 1982 };
