/** Synthetic VND-26Q3-167 evidence contract. */
export interface VendorEvaluation { id: string; vendor: string; reviewDate: string; trialP95Ms: number; availabilityPct: number; evidenceCoveragePct: number; recommendation: string; }

export const evaluation1846: VendorEvaluation = {
  id: 'VND-26Q3-167', vendor: 'Platform Meridian', reviewDate: '2026-08-25', trialP95Ms: 179,
  availabilityPct: 99.94, evidenceCoveragePct: 97, recommendation: 'request evidence clarification'
};

export const meetsPlanningThreshold1846 = (item: VendorEvaluation): boolean => item.trialP95Ms <= 200 && item.evidenceCoveragePct >= 80;
