/** Synthetic VND-26Q3-190 evidence contract. */
export interface VendorEvaluation { id: string; vendor: string; reviewDate: string; trialP95Ms: number; availabilityPct: number; evidenceCoveragePct: number; recommendation: string; }

export const evaluation1869: VendorEvaluation = {
  id: 'VND-26Q3-190', vendor: 'Platform Juniper', reviewDate: '2026-07-24', trialP95Ms: 153,
  availabilityPct: 99.80, evidenceCoveragePct: 78, recommendation: 'retain for comparative review'
};

export const meetsPlanningThreshold1869 = (item: VendorEvaluation): boolean => item.trialP95Ms <= 200 && item.evidenceCoveragePct >= 80;
