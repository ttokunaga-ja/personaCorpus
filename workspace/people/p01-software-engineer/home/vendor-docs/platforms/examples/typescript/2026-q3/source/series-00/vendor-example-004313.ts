/** Synthetic VND-26Q3-234 evidence contract. */
export interface VendorEvaluation { id: string; vendor: string; reviewDate: string; trialP95Ms: number; availabilityPct: number; evidenceCoveragePct: number; recommendation: string; }

export const evaluation4313: VendorEvaluation = {
  id: 'VND-26Q3-234', vendor: 'Platform Juniper', reviewDate: '2026-09-20', trialP95Ms: 147,
  availabilityPct: 99.88, evidenceCoveragePct: 86, recommendation: 'retain for comparative review'
};

export const meetsPlanningThreshold4313 = (item: VendorEvaluation): boolean => item.trialP95Ms <= 200 && item.evidenceCoveragePct >= 80;
