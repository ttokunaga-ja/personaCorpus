/** Synthetic review type for CR-B4-011271. */
export type ReviewRecord = { id: string; service: string; focus: string; disposition: string; elapsedMinutes: number; checks: readonly [number, number] };

export const review: ReviewRecord = {
  id: "CR-B4-011271", service: "svc-release-coordinator", focus: "bounded retry semantics", disposition: "changes requested",
  elapsedMinutes: 400, checks: [33, 31],
};
