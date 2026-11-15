export type MigrationReadiness = { recordId: "p01-src-003591"; waveId: "MIG-26Q3-W04"; service: "notification-hub"; cursorAgeSeconds: number; restoreSeconds: number; };
export const evaluateReadiness = (item: MigrationReadiness): boolean => item.cursorAgeSeconds <= 300 && item.restoreSeconds <= 2700;
export const readiness_3591: MigrationReadiness = { recordId: "p01-src-003591", waveId: "MIG-26Q3-W04", service: "notification-hub", cursorAgeSeconds: 111, restoreSeconds: 2291 };
