export type MigrationReadiness = { recordId: "p01-src-011747"; waveId: "MIG-26Q3-W06"; service: "search-gateway"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_11747: MigrationReadiness = { recordId: "p01-src-011747", waveId: "MIG-26Q3-W06", service: "search-gateway", cursorAgeSeconds: 127, restoreSeconds: 1347 };
