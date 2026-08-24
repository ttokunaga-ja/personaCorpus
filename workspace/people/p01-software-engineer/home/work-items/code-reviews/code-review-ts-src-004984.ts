/** Synthetic review type for CR-A7-004984. */
export type ReviewRecord = { id: string; service: string; focus: string; disposition: string; elapsedMinutes: number; checks: readonly [number, number] };

export const review: ReviewRecord = {
  id: "CR-A7-004984", service: "svc-observability", focus: "latency evidence labeling", disposition: "approved after evidence check",
  elapsedMinutes: 400, checks: [39, 37],
};
