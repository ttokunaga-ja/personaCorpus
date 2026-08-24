/** Synthetic review type for CR-A7-006490. */
export type ReviewRecord = { id: string; service: string; focus: string; disposition: string; elapsedMinutes: number; checks: readonly [number, number] };

export const review: ReviewRecord = {
  id: "CR-A7-006490", service: "svc-edge-api", focus: "review-thread ownership", disposition: "approved",
  elapsedMinutes: 400, checks: [37, 35],
};
