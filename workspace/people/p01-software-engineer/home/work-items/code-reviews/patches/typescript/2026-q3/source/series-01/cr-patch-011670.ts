/** Synthetic review type for CR-B4-011670. */
export type ReviewRecord = { id: string; service: string; focus: string; disposition: string; elapsedMinutes: number; checks: readonly [number, number] };

export const review: ReviewRecord = {
  id: "CR-B4-011670", service: "svc-edge-api", focus: "review-thread ownership", disposition: "approved",
  elapsedMinutes: 400, checks: [26, 24],
};
