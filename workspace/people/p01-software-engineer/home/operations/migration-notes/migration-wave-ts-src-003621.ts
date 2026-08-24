export type MigrationReadiness = { recordId: "p01-src-003621"; waveId: "MIG-26Q3-W04"; service: "event-bus"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_3621: MigrationReadiness = { recordId: "p01-src-003621", waveId: "MIG-26Q3-W04", service: "event-bus", cursorAgeSeconds: 141, restoreSeconds: 2321 };
