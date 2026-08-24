/** Synthetic VND-26Q3-213 evidence contract. */
export interface VendorEvaluation { id: string; vendor: string; reviewDate: string; trialP95Ms: number; availabilityPct: number; evidenceCoveragePct: number; recommendation: string; }

export const evaluation1892: VendorEvaluation = {
  id: 'VND-26Q3-213', vendor: 'Platform Helix', reviewDate: '2026-09-23', trialP95Ms: 176,
  availabilityPct: 99.91, evidenceCoveragePct: 80, recommendation: 'advance to controlled trial'
};

export const meetsPlanningThreshold1892 = (item: VendorEvaluation): boolean => item.trialP95Ms <= 200 && item.evidenceCoveragePct >= 80;
