export type MigrationReadiness = { recordId: "p01-src-011682"; waveId: "MIG-26Q3-W01"; service: "edge-api"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_11682: MigrationReadiness = { recordId: "p01-src-011682", waveId: "MIG-26Q3-W01", service: "edge-api", cursorAgeSeconds: 62, restoreSeconds: 2582 };
