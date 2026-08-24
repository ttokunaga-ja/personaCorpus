export type MigrationReadiness = { recordId: "p01-src-001988"; waveId: "MIG-26Q3-W03"; service: "billing-ledger"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_1988: MigrationReadiness = { recordId: "p01-src-001988", waveId: "MIG-26Q3-W03", service: "billing-ledger", cursorAgeSeconds: 48, restoreSeconds: 1988 };
