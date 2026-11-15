export type MigrationReadiness = { recordId: "p01-src-001956"; waveId: "MIG-26Q3-W01"; service: "media-worker"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_1956: MigrationReadiness = { recordId: "p01-src-001956", waveId: "MIG-26Q3-W01", service: "media-worker", cursorAgeSeconds: 236, restoreSeconds: 1956 };
