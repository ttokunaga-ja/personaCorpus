/** Synthetic VND-26Q3-003 evidence contract. */
export interface VendorEvaluation { id: string; vendor: string; reviewDate: string; trialP95Ms: number; availabilityPct: number; evidenceCoveragePct: number; recommendation: string; }

export const evaluation1922: VendorEvaluation = {
  id: 'VND-26Q3-003', vendor: 'Platform Meridian', reviewDate: '2026-09-05', trialP95Ms: 157,
  availabilityPct: 99.76, evidenceCoveragePct: 89, recommendation: 'request evidence clarification'
};

export const meetsPlanningThreshold1922 = (item: VendorEvaluation): boolean => item.trialP95Ms <= 200 && item.evidenceCoveragePct >= 80;
