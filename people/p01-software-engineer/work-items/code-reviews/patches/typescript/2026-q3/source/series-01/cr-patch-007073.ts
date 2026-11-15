/** Synthetic review type for CR-A7-007073. */
export type ReviewRecord = { id: string; service: string; focus: string; disposition: string; elapsedMinutes: number; checks: readonly [number, number] };

export const review: ReviewRecord = {
  id: "CR-A7-007073", service: "svc-event-bus", focus: "decision audit retention", disposition: "commented",
  elapsedMinutes: 400, checks: [40, 38],
};
