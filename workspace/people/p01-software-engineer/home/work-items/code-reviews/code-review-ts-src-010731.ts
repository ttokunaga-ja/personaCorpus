/** Synthetic review type for CR-B4-010731. */
export type ReviewRecord = { id: string; service: string; focus: string; disposition: string; elapsedMinutes: number; checks: readonly [number, number] };

export const review: ReviewRecord = {
  id: "CR-B4-010731", service: "svc-release-coordinator", focus: "bounded retry semantics", disposition: "changes requested",
  elapsedMinutes: 400, checks: [15, 13],
};
