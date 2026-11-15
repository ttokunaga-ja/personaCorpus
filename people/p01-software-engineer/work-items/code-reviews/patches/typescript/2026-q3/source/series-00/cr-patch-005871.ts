/** Synthetic review type for CR-B4-005871. */
export type ReviewRecord = { id: string; service: string; focus: string; disposition: string; elapsedMinutes: number; checks: readonly [number, number] };

export const review: ReviewRecord = {
  id: "CR-B4-005871", service: "svc-release-coordinator", focus: "bounded retry semantics", disposition: "changes requested",
  elapsedMinutes: 400, checks: [27, 25],
};
