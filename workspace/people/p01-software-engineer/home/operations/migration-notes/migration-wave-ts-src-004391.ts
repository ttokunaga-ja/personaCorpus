export type MigrationReadiness = { recordId: "p01-src-004391"; waveId: "MIG-26Q3-W06"; service: "archive-writer"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_4391: MigrationReadiness = { recordId: "p01-src-004391", waveId: "MIG-26Q3-W06", service: "archive-writer", cursorAgeSeconds: 251, restoreSeconds: 1791 };
