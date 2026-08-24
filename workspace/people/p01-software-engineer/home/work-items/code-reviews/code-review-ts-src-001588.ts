/** Synthetic code-review contract for CR-A7-001588. */
export const reviewId = 'CR-A7-001588';
export const reviewElapsedMinutes = 400;

export type ReviewEvidence = {
  reviewId: string; service: string; focus: string; disposition: string;
  checks: { executed: number; passed: number }; reviewElapsedMinutes: number;
};

export const reviewEvidence: ReviewEvidence = {
  reviewId, service: 'svc-event-bus', focus: 'latency budget accounting', disposition: 'commented',
  checks: { executed: 19, passed: 18 }, reviewElapsedMinutes,
};
