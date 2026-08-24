export type MigrationReadiness = { recordId: "p01-src-010351"; waveId: "MIG-26Q3-W02"; service: "release-coordinator"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_10351: MigrationReadiness = { recordId: "p01-src-010351", waveId: "MIG-26Q3-W02", service: "release-coordinator", cursorAgeSeconds: 51, restoreSeconds: 2551 };
