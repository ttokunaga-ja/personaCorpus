/** Synthetic VND-26Q3-230 evidence contract. */
export interface VendorEvaluation { id: string; vendor: string; reviewDate: string; trialP95Ms: number; availabilityPct: number; evidenceCoveragePct: number; recommendation: string; }

export const evaluation1909: VendorEvaluation = {
  id: 'VND-26Q3-230', vendor: 'Platform Juniper', reviewDate: '2026-08-16', trialP95Ms: 193,
  availabilityPct: 99.85, evidenceCoveragePct: 97, recommendation: 'retain for comparative review'
};

export const meetsPlanningThreshold1909 = (item: VendorEvaluation): boolean => item.trialP95Ms <= 200 && item.evidenceCoveragePct >= 80;
