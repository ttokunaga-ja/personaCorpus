export type MigrationReadiness = { recordId: "p01-src-008782"; waveId: "MIG-26Q3-W05"; service: "policy-cache"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_8782: MigrationReadiness = { recordId: "p01-src-008782", waveId: "MIG-26Q3-W05", service: "policy-cache", cursorAgeSeconds: 242, restoreSeconds: 2282 };
