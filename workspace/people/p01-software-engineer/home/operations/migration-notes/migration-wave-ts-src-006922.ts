export type MigrationReadiness = { recordId: "p01-src-006922"; waveId: "MIG-26Q3-W05"; service: "audit-journal"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_6922: MigrationReadiness = { recordId: "p01-src-006922", waveId: "MIG-26Q3-W05", service: "audit-journal", cursorAgeSeconds: 142, restoreSeconds: 1722 };
