/** Synthetic review type for CR-B4-008730. */
export type ReviewRecord = { id: string; service: string; focus: string; disposition: string; elapsedMinutes: number; checks: readonly [number, number] };

export const review: ReviewRecord = {
  id: "CR-B4-008730", service: "svc-edge-api", focus: "review-thread ownership", disposition: "approved",
  elapsedMinutes: 400, checks: [15, 13],
};
