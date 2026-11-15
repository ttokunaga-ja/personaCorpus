export type MigrationReadiness = { recordId: "p01-src-008382"; waveId: "MIG-26Q3-W01"; service: "media-worker"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_8382: MigrationReadiness = { recordId: "p01-src-008382", waveId: "MIG-26Q3-W01", service: "media-worker", cursorAgeSeconds: 62, restoreSeconds: 1882 };
