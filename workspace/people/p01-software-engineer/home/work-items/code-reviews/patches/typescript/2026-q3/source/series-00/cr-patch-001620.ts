/** Synthetic code-review contract for CR-A7-001620. */
export const reviewId = 'CR-A7-001620';
export const reviewElapsedMinutes = 400;

export type ReviewEvidence = {
  reviewId: string; service: string; focus: string; disposition: string;
  checks: { executed: number; passed: number }; reviewElapsedMinutes: number;
};

export const reviewEvidence: ReviewEvidence = {
  reviewId, service: 'svc-edge-api', focus: 'error boundary propagation', disposition: 'approved with notes',
  checks: { executed: 20, passed: 19 }, reviewElapsedMinutes,
};
