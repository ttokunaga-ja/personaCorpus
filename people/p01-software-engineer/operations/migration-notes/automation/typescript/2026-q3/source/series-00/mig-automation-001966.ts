export type MigrationReadiness = { recordId: "p01-src-001966"; waveId: "MIG-26Q3-W05"; service: "observability"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_1966: MigrationReadiness = { recordId: "p01-src-001966", waveId: "MIG-26Q3-W05", service: "observability", cursorAgeSeconds: 246, restoreSeconds: 1966 };
