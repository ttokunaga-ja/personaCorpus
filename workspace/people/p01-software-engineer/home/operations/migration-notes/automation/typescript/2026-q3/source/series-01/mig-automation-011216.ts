export type MigrationReadiness = { recordId: "p01-src-011216"; waveId: "MIG-26Q3-W03"; service: "decision-store"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_11216: MigrationReadiness = { recordId: "p01-src-011216", waveId: "MIG-26Q3-W03", service: "decision-store", cursorAgeSeconds: 256, restoreSeconds: 2116 };
