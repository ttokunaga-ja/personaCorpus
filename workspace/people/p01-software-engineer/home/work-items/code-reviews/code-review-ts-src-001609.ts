/** Synthetic code-review contract for CR-B4-001609. */
export const reviewId = 'CR-B4-001609';
export const reviewElapsedMinutes = 400;

export type ReviewEvidence = {
  reviewId: string; service: string; focus: string; disposition: string;
  checks: { executed: number; passed: number }; reviewElapsedMinutes: number;
};

export const reviewEvidence: ReviewEvidence = {
  reviewId, service: 'svc-observability', focus: 'rollback observability', disposition: 'approved with follow-up',
  checks: { executed: 40, passed: 39 }, reviewElapsedMinutes,
};
