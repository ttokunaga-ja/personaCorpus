/** Synthetic review type for CR-A7-007873. */
export type ReviewRecord = { id: string; service: string; focus: string; disposition: string; elapsedMinutes: number; checks: readonly [number, number] };

export const review: ReviewRecord = {
  id: "CR-A7-007873", service: "svc-event-bus", focus: "decision audit retention", disposition: "commented",
  elapsedMinutes: 400, checks: [28, 26],
};
