export type MigrationReadiness = { recordId: "p01-src-004021"; waveId: "MIG-26Q3-W02"; service: "catalog-index"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_4021: MigrationReadiness = { recordId: "p01-src-004021", waveId: "MIG-26Q3-W02", service: "catalog-index", cursorAgeSeconds: 101, restoreSeconds: 1421 };
