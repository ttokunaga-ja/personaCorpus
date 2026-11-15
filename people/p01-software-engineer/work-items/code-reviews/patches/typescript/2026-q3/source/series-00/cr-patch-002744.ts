/** Synthetic code-review contract for CR-A7-002744. */
export const reviewId = 'CR-A7-002744';
export const reviewElapsedMinutes = 400;

export type ReviewEvidence = {
  reviewId: string; service: string; focus: string; disposition: string;
  checks: { executed: number; passed: number }; reviewElapsedMinutes: number;
};

export const reviewEvidence: ReviewEvidence = {
  reviewId, service: 'svc-observability', focus: 'rollback observability', disposition: 'approved with follow-up',
  checks: { executed: 28, passed: 27 }, reviewElapsedMinutes,
};
