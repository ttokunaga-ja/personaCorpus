/** HelioDesk first-response SLA helper. Times are ISO-8601 instants. */
export const FIRST_RESPONSE_SLA_MINUTES = 4 * 60;
export const AT_RISK_WINDOW_MINUTES = 60;

export type SlaStatus = "within" | "at-risk" | "breached" | "met";

export interface SlaAssessment {
  deadline: Date;
  status: SlaStatus;
  minutesRemaining: number;
}

function assertValidDate(date: Date, label: string): void {
  if (Number.isNaN(date.getTime())) throw new Error(`${label} must be a valid Date`);
}

export function firstResponseDeadline(createdAt: Date, slaMinutes = FIRST_RESPONSE_SLA_MINUTES): Date {
  assertValidDate(createdAt, "createdAt");
  if (!Number.isFinite(slaMinutes) || slaMinutes < 0) throw new Error("slaMinutes must be a non-negative finite number");
  return new Date(createdAt.getTime() + slaMinutes * 60_000);
}

export function assessFirstResponse(
  createdAt: Date,
  now: Date,
  respondedAt?: Date,
  slaMinutes = FIRST_RESPONSE_SLA_MINUTES,
  atRiskWindowMinutes = AT_RISK_WINDOW_MINUTES,
): SlaAssessment {
  assertValidDate(now, "now");
  if (respondedAt) assertValidDate(respondedAt, "respondedAt");
  if (!Number.isFinite(atRiskWindowMinutes) || atRiskWindowMinutes < 0) throw new Error("atRiskWindowMinutes must be non-negative");
  const deadline = firstResponseDeadline(createdAt, slaMinutes);
  const minutesRemaining = Math.floor((deadline.getTime() - now.getTime()) / 60_000);
  if (respondedAt && respondedAt.getTime() <= deadline.getTime()) return { deadline, status: "met", minutesRemaining };
  if (now.getTime() > deadline.getTime()) return { deadline, status: "breached", minutesRemaining };
  if (minutesRemaining <= atRiskWindowMinutes) return { deadline, status: "at-risk", minutesRemaining };
  return { deadline, status: "within", minutesRemaining };
}
