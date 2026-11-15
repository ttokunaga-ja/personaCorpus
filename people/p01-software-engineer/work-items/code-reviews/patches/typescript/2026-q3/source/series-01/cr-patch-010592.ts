/** Synthetic review type for CR-A7-010592. */
export type ReviewRecord = { id: string; service: string; focus: string; disposition: string; elapsedMinutes: number; checks: readonly [number, number] };

export const review: ReviewRecord = {
  id: "CR-A7-010592", service: "svc-decision-store", focus: "contract version visibility", disposition: "approved with note",
  elapsedMinutes: 400, checks: [21, 19],
};
