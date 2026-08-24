export type MigrationReadiness = { recordId: "p01-src-001932"; waveId: "MIG-26Q3-W01"; service: "identity-sync"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_1932: MigrationReadiness = { recordId: "p01-src-001932", waveId: "MIG-26Q3-W01", service: "identity-sync", cursorAgeSeconds: 212, restoreSeconds: 1932 };
