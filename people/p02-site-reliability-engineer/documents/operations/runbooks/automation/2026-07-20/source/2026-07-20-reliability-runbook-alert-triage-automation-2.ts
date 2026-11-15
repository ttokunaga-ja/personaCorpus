// Northstar Transit Cloud | Checkout SVC-17 | 2026-07-13 | error budget 2.4% | INC-260713
export const topic = "alert_triage";
export const fingerprint = "6484d89d504aa1d7";
export const alertLevel = (depth) => depth >= 24000 ? 'critical' : depth >= 12000 ? 'warning' : 'normal';
