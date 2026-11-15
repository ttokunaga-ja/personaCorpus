/** Synthetic VND-26Q3-146 evidence contract. */
export interface VendorEvaluation { id: string; vendor: string; reviewDate: string; trialP95Ms: number; availabilityPct: number; evidenceCoveragePct: number; recommendation: string; }

export const evaluation1825: VendorEvaluation = {
  id: 'VND-26Q3-146', vendor: 'Platform Juniper', reviewDate: '2026-08-04', trialP95Ms: 158,
  availabilityPct: 99.72, evidenceCoveragePct: 97, recommendation: 'retain for comparative review'
};

export const meetsPlanningThreshold1825 = (item: VendorEvaluation): boolean => item.trialP95Ms <= 200 && item.evidenceCoveragePct >= 80;
