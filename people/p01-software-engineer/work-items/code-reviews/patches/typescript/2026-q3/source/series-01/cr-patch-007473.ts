/** Synthetic review type for CR-B4-007473. */
export type ReviewRecord = { id: string; service: string; focus: string; disposition: string; elapsedMinutes: number; checks: readonly [number, number] };

export const review: ReviewRecord = {
  id: "CR-B4-007473", service: "svc-event-bus", focus: "decision audit retention", disposition: "commented",
  elapsedMinutes: 400, checks: [34, 32],
};
