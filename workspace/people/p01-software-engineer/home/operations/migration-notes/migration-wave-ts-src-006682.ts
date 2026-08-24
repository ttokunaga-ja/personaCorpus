export type MigrationReadiness = { recordId: "p01-src-006682"; waveId: "MIG-26Q3-W05"; service: "observability"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_6682: MigrationReadiness = { recordId: "p01-src-006682", waveId: "MIG-26Q3-W05", service: "observability", cursorAgeSeconds: 122, restoreSeconds: 1482 };
