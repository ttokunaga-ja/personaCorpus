/** Synthetic review type for CR-A7-009530. */
export type ReviewRecord = { id: string; service: string; focus: string; disposition: string; elapsedMinutes: number; checks: readonly [number, number] };

export const review: ReviewRecord = {
  id: "CR-A7-009530", service: "svc-edge-api", focus: "review-thread ownership", disposition: "approved",
  elapsedMinutes: 400, checks: [32, 30],
};
