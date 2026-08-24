/** Synthetic VND-26Q3-087 evidence contract. */
export interface VendorEvaluation { id: string; vendor: string; reviewDate: string; trialP95Ms: number; availabilityPct: number; evidenceCoveragePct: number; recommendation: string; }

export const evaluation3686: VendorEvaluation = {
  id: 'VND-26Q3-087', vendor: 'Platform Meridian', reviewDate: '2026-09-17', trialP95Ms: 157,
  availabilityPct: 99.74, evidenceCoveragePct: 89, recommendation: 'request evidence clarification'
};

export const meetsPlanningThreshold3686 = (item: VendorEvaluation): boolean => item.trialP95Ms <= 200 && item.evidenceCoveragePct >= 80;
