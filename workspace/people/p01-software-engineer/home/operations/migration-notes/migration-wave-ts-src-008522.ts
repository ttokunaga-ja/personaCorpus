export type MigrationReadiness = { recordId: "p01-src-008522"; waveId: "MIG-26Q3-W03"; service: "billing-ledger"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_8522: MigrationReadiness = { recordId: "p01-src-008522", waveId: "MIG-26Q3-W03", service: "billing-ledger", cursorAgeSeconds: 202, restoreSeconds: 2022 };
