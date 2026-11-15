/** Synthetic review type for CR-B4-006747. */
export type ReviewRecord = { id: string; service: string; focus: string; disposition: string; elapsedMinutes: number; checks: readonly [number, number] };

export const review: ReviewRecord = {
  id: "CR-B4-006747", service: "svc-decision-store", focus: "contract version visibility", disposition: "approved with note",
  elapsedMinutes: 400, checks: [33, 31],
};
