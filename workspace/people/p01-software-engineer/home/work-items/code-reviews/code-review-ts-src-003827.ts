/** Synthetic code-review contract for CR-B4-003827. */
export const reviewId = 'CR-B4-003827';
export const reviewElapsedMinutes = 400;

export type ReviewEvidence = {
  reviewId: string; service: string; focus: string; disposition: string;
  checks: { executed: number; passed: number }; reviewElapsedMinutes: number;
};

export const reviewEvidence: ReviewEvidence = {
  reviewId, service: 'svc-decision-store', focus: 'schema compatibility', disposition: 'approved',
  checks: { executed: 26, passed: 25 }, reviewElapsedMinutes,
};
