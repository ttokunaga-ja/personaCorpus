type Priority = "P1" | "P2" | "P3" | "P4";
type SlaState = "within" | "at-risk" | "breached";

export function classifySla(openedAtIso: string, nowIso: string, targetHours = 4): SlaState {
  const elapsedHours = (Date.parse(nowIso) - Date.parse(openedAtIso)) / 3_600_000;
  if (!Number.isFinite(elapsedHours) || targetHours <= 0) throw new Error("invalid SLA timestamp or target");
  if (elapsedHours > targetHours) return "breached";
  if (elapsedHours >= targetHours * 0.75) return "at-risk";
  return "within";
}

export function escalationLabel(priority: Priority, sla: SlaState): string {
  if (priority === "P1") return "Lead bridge / 30-minute update";
  if (sla === "breached") return "Recovery owner and customer update now";
  if (sla === "at-risk") return "Acknowledge before 4-hour target";
  return "Owner-led next action";
}

const snapshot = "2026-08-04T09:00:00Z";
console.log(classifySla("2026-08-04T02:14:00Z", snapshot)); // HD-260804-2718: breached
console.log(escalationLabel("P1", "breached"));
