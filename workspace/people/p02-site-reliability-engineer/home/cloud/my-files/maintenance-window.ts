/** Build a reviewable maintenance-window plan; no scheduling API is called. */

type MaintenanceWindow = {
  service: string;
  startUtc: string;
  durationMinutes: number;
  changeId: string;
  rollbackTrigger: string;
};

type PlannedWindow = MaintenanceWindow & { endUtc: string; approval: "required" };

function parseUtc(value: string): Date {
  const date = new Date(value);
  if (!value.endsWith("Z") || Number.isNaN(date.getTime())) {
    throw new Error(`Expected an ISO-8601 UTC timestamp ending in Z: ${value}`);
  }
  return date;
}

export function planWindow(window: MaintenanceWindow): PlannedWindow {
  if (!window.service.trim() || !window.changeId.trim() || !window.rollbackTrigger.trim()) {
    throw new Error("service, changeId, and rollbackTrigger are required");
  }
  if (!Number.isInteger(window.durationMinutes) || window.durationMinutes < 15 || window.durationMinutes > 240) {
    throw new Error("durationMinutes must be a whole number between 15 and 240");
  }
  const start = parseUtc(window.startUtc);
  const end = new Date(start.getTime() + window.durationMinutes * 60_000);
  return { ...window, startUtc: start.toISOString(), endUtc: end.toISOString(), approval: "required" };
}

const sample = planWindow({
  service: "Checkout SVC-17",
  startUtc: "2026-07-20T02:00:00Z",
  durationMinutes: 45,
  changeId: "CHG-260720-01",
  rollbackTrigger: "Checkout availability below 99.5% for five consecutive minutes",
});

console.log(JSON.stringify({ organization: "Northstar Transit Cloud", window: sample }, null, 2));
