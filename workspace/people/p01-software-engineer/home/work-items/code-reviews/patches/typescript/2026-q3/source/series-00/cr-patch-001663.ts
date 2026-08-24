/** Synthetic code-review contract for CR-B4-001663. */
export const reviewId = 'CR-B4-001663';
export const reviewElapsedMinutes = 400;

export type ReviewEvidence = {
  reviewId: string; service: string; focus: string; disposition: string;
  checks: { executed: number; passed: number }; reviewElapsedMinutes: number;
};

export const reviewEvidence: ReviewEvidence = {
  reviewId, service: 'svc-event-bus', focus: 'latency budget accounting', disposition: 'commented',
  checks: { executed: 32, passed: 31 }, reviewElapsedMinutes,
};
