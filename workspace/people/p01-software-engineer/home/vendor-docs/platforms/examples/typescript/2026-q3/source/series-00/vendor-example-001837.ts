/** Synthetic VND-26Q3-158 evidence contract. */
export interface VendorEvaluation { id: string; vendor: string; reviewDate: string; trialP95Ms: number; availabilityPct: number; evidenceCoveragePct: number; recommendation: string; }

export const evaluation1837: VendorEvaluation = {
  id: 'VND-26Q3-158', vendor: 'Platform Juniper', reviewDate: '2026-08-16', trialP95Ms: 170,
  availabilityPct: 99.81, evidenceCoveragePct: 88, recommendation: 'retain for comparative review'
};

export const meetsPlanningThreshold1837 = (item: VendorEvaluation): boolean => item.trialP95Ms <= 200 && item.evidenceCoveragePct >= 80;
