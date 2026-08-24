/** Synthetic review type for CR-B4-011109. */
export type ReviewRecord = { id: string; service: string; focus: string; disposition: string; elapsedMinutes: number; checks: readonly [number, number] };

export const review: ReviewRecord = {
  id: "CR-B4-011109", service: "svc-observability", focus: "latency evidence labeling", disposition: "approved after evidence check",
  elapsedMinutes: 400, checks: [16, 14],
};
