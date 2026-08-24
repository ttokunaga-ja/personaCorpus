/** Synthetic VND-26Q3-222 evidence contract. */
export interface VendorEvaluation { id: string; vendor: string; reviewDate: string; trialP95Ms: number; availabilityPct: number; evidenceCoveragePct: number; recommendation: string; }

export const evaluation1901: VendorEvaluation = {
  id: 'VND-26Q3-222', vendor: 'Platform Juniper', reviewDate: '2026-09-08', trialP95Ms: 185,
  availabilityPct: 99.79, evidenceCoveragePct: 89, recommendation: 'retain for comparative review'
};

export const meetsPlanningThreshold1901 = (item: VendorEvaluation): boolean => item.trialP95Ms <= 200 && item.evidenceCoveragePct >= 80;
