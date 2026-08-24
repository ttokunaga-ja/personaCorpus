export type MigrationReadiness = { recordId: "p01-src-010947"; waveId: "MIG-26Q3-W04"; service: "event-bus"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_10947: MigrationReadiness = { recordId: "p01-src-010947", waveId: "MIG-26Q3-W04", service: "event-bus", cursorAgeSeconds: 207, restoreSeconds: 1847 };
