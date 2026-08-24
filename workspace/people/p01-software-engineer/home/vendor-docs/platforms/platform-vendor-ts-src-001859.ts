/** Synthetic VND-26Q3-180 evidence contract. */
export interface VendorEvaluation { id: string; vendor: string; reviewDate: string; trialP95Ms: number; availabilityPct: number; evidenceCoveragePct: number; recommendation: string; }

export const evaluation1859: VendorEvaluation = {
  id: 'VND-26Q3-180', vendor: 'Platform Quartz', reviewDate: '2026-09-14', trialP95Ms: 192,
  availabilityPct: 99.85, evidenceCoveragePct: 89, recommendation: 'hold pending interface validation'
};

export const meetsPlanningThreshold1859 = (item: VendorEvaluation): boolean => item.trialP95Ms <= 200 && item.evidenceCoveragePct >= 80;
