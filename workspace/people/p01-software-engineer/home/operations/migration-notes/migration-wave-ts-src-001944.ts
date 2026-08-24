export type MigrationReadiness = { recordId: "p01-src-001944"; waveId: "MIG-26Q3-W01"; service: "edge-api"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_1944: MigrationReadiness = { recordId: "p01-src-001944", waveId: "MIG-26Q3-W01", service: "edge-api", cursorAgeSeconds: 224, restoreSeconds: 1944 };
