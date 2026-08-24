/** Synthetic code-review contract for CR-A7-001598. */
export const reviewId = 'CR-A7-001598';
export const reviewElapsedMinutes = 400;

export type ReviewEvidence = {
  reviewId: string; service: string; focus: string; disposition: string;
  checks: { executed: number; passed: number }; reviewElapsedMinutes: number;
};

export const reviewEvidence: ReviewEvidence = {
  reviewId, service: 'svc-event-bus', focus: 'latency budget accounting', disposition: 'commented',
  checks: { executed: 29, passed: 28 }, reviewElapsedMinutes,
};
