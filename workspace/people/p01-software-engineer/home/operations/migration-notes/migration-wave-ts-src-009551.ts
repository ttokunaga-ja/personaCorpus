export type MigrationReadiness = { recordId: "p01-src-009551"; waveId: "MIG-26Q3-W06"; service: "search-gateway"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_9551: MigrationReadiness = { recordId: "p01-src-009551", waveId: "MIG-26Q3-W06", service: "search-gateway", cursorAgeSeconds: 131, restoreSeconds: 1751 };
