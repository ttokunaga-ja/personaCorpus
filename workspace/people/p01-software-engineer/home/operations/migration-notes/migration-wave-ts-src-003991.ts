export type MigrationReadiness = { recordId: "p01-src-003991"; waveId: "MIG-26Q3-W02"; service: "reporting-api"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_3991: MigrationReadiness = { recordId: "p01-src-003991", waveId: "MIG-26Q3-W02", service: "reporting-api", cursorAgeSeconds: 71, restoreSeconds: 1391 };
