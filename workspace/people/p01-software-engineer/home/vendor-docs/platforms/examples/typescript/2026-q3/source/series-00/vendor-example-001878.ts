/** Synthetic VND-26Q3-199 evidence contract. */
export interface VendorEvaluation { id: string; vendor: string; reviewDate: string; trialP95Ms: number; availabilityPct: number; evidenceCoveragePct: number; recommendation: string; }

export const evaluation1878: VendorEvaluation = {
  id: 'VND-26Q3-199', vendor: 'Platform Meridian', reviewDate: '2026-07-09', trialP95Ms: 162,
  availabilityPct: 99.93, evidenceCoveragePct: 87, recommendation: 'request evidence clarification'
};

export const meetsPlanningThreshold1878 = (item: VendorEvaluation): boolean => item.trialP95Ms <= 200 && item.evidenceCoveragePct >= 80;
